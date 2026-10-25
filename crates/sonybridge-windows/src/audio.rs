use thiserror::Error;
use windows::core::{GUID, HSTRING, PCWSTR};
use windows::Win32::Media::Audio::{
    eCommunications, eConsole, eMultimedia, eRender, ERole, IMMDeviceEnumerator,
    MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_MULTITHREADED,
};

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("COM initialization error: {0}")]
    Com(#[from] windows::core::Error),
    #[error("Endpoint not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone)]
pub struct AudioEndpoint {
    pub id: String,
    pub name: String,
    pub is_hands_free: bool,
    pub is_default: bool,
}

/// Undocumented Windows Core Audio `IPolicyConfig` vtable for programmatic default audio endpoint switching.
#[repr(C)]
struct IPolicyConfigVtbl {
    pub base: windows::core::IUnknown_Vtbl,
    pub get_mix_format: usize,
    pub get_device_format: usize,
    pub reset_device_format: usize,
    pub set_device_format: usize,
    pub get_processing_period: usize,
    pub set_processing_period: usize,
    pub get_share_mode: usize,
    pub set_share_mode: usize,
    pub get_property_value: usize,
    pub set_property_value: usize,
    pub set_default_endpoint:
        unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR, ERole) -> windows::core::HRESULT,
    pub set_endpoint_visibility: usize,
}

const CLSID_POLICY_CONFIG_CLIENT: GUID = GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);
const IID_IPOLICY_CONFIG: GUID = GUID::from_u128(0xf8679f50_850a_41cf_9c72_430f290290c8);

pub struct AudioEndpointSanitizer;

impl AudioEndpointSanitizer {
    /// Enumerate all active audio render endpoints on Windows.
    pub fn enumerate_render_endpoints() -> Result<Vec<AudioEndpoint>, AudioError> {
        let mut endpoints = Vec::new();

        // SAFETY: All COM interfaces are scoped inside the inner block so their Drop
        // implementations run before CoUninitialize(). PWSTR allocations are freed with CoTaskMemFree.
        unsafe {
            let com_init = CoInitializeEx(None, COINIT_MULTITHREADED);
            {
                let enumerator: IMMDeviceEnumerator =
                    CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

                let default_id = enumerator
                    .GetDefaultAudioEndpoint(eRender, eConsole)
                    .ok()
                    .and_then(|dev| dev.GetId().ok())
                    .map(|pwstr| {
                        let s = pwstr.to_string().unwrap_or_default();
                        CoTaskMemFree(Some(pwstr.as_ptr() as *const _));
                        s
                    })
                    .unwrap_or_default();

                let collection = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
                let count = collection.GetCount()?;

                for i in 0..count {
                    if let Ok(device) = collection.Item(i) {
                        if let Ok(pwstr) = device.GetId() {
                            let id = pwstr.to_string().unwrap_or_default();
                            CoTaskMemFree(Some(pwstr.as_ptr() as *const _));

                            let lower_id = id.to_lowercase();
                            let is_hands_free = lower_id.contains("hands-free")
                                || lower_id.contains("bthhfenum")
                                || lower_id.contains("0000111e");
                            let is_default = !default_id.is_empty() && id == default_id;

                            endpoints.push(AudioEndpoint {
                                id: id.clone(),
                                name: if is_hands_free {
                                    format!("Hands-Free AG Endpoint #{i}")
                                } else {
                                    format!("Stereo Render Endpoint #{i}")
                                },
                                is_hands_free,
                                is_default,
                            });
                        }
                    }
                }
            }

            if com_init.is_ok() {
                CoUninitialize();
            }
        }

        Ok(endpoints)
    }

    /// Check if Hands-Free audio is currently active for Sony headphones.
    pub fn is_hands_free_active() -> bool {
        match Self::enumerate_render_endpoints() {
            Ok(endpoints) => endpoints
                .iter()
                .any(|e| e.is_hands_free && (e.is_default || endpoints.len() > 1)),
            Err(_) => false,
        }
    }

    /// Programmatically set the first non-Hands-Free Stereo A2DP endpoint as the Windows default
    /// for Console, Multimedia, and Communications roles via `IPolicyConfig`.
    pub fn enforce_stereo_a2dp() -> Result<String, AudioError> {
        let endpoints = Self::enumerate_render_endpoints()?;
        let target = endpoints
            .iter()
            .find(|e| !e.is_hands_free)
            .ok_or_else(|| AudioError::NotFound("No Stereo A2DP render endpoint found".into()))?;

        // SAFETY: Uses CoCreateInstance with CLSID_PolicyConfigClient and IID_IPolicyConfig,
        // invokes SetDefaultEndpoint for all three audio roles, and releases the COM interface
        // prior to CoUninitialize().
        unsafe {
            let com_init = CoInitializeEx(None, COINIT_MULTITHREADED);
            {
                let mut raw_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
                let hr = windows::Win32::System::Com::CoCreateInstance(
                    &CLSID_POLICY_CONFIG_CLIENT,
                    None,
                    CLSCTX_ALL,
                )
                .and_then(|unk: windows::core::IUnknown| {
                    windows::core::Interface::query(&unk, &IID_IPOLICY_CONFIG, &mut raw_ptr).ok()
                });

                if hr.is_ok() && !raw_ptr.is_null() {
                    let vtbl = *(raw_ptr as *const *const IPolicyConfigVtbl);
                    let hstr = HSTRING::from(&target.id);
                    let pcwstr = PCWSTR(hstr.as_ptr());

                    let _ = ((*vtbl).set_default_endpoint)(raw_ptr, pcwstr, eConsole);
                    let _ = ((*vtbl).set_default_endpoint)(raw_ptr, pcwstr, eMultimedia);
                    let _ = ((*vtbl).set_default_endpoint)(raw_ptr, pcwstr, eCommunications);
                    let _ = ((*vtbl).base.Release)(raw_ptr);
                }
            }
            if com_init.is_ok() {
                CoUninitialize();
            }
        }

        Ok(target.id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_render_endpoints_runs_safely() {
        let result = AudioEndpointSanitizer::enumerate_render_endpoints();
        assert!(result.is_ok());
    }
}
