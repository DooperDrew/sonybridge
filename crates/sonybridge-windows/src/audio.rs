use thiserror::Error;
use windows::Win32::Media::Audio::{
    eRender, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
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
}

pub struct AudioEndpointSanitizer;

impl AudioEndpointSanitizer {
    /// Enumerate all active audio render endpoints on Windows.
    pub fn enumerate_render_endpoints() -> Result<Vec<AudioEndpoint>, AudioError> {
        let mut endpoints = Vec::new();

        unsafe {
            let com_init = CoInitializeEx(None, COINIT_MULTITHREADED);
            {
                let enumerator: IMMDeviceEnumerator =
                    CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

                let collection = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
                let count = collection.GetCount()?;

                for i in 0..count {
                    if let Ok(device) = collection.Item(i) {
                        if let Ok(pwstr) = device.GetId() {
                            let id = pwstr.to_string().unwrap_or_default();
                            windows::Win32::System::Com::CoTaskMemFree(Some(
                                pwstr.as_ptr() as *const _
                            ));

                            let is_hands_free = id.to_lowercase().contains("hands-free")
                                || id.to_lowercase().contains("bthhfenum");

                            endpoints.push(AudioEndpoint {
                                id: id.clone(),
                                name: format!("Endpoint #{i}"),
                                is_hands_free,
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
            Ok(endpoints) => endpoints.iter().any(|e| {
                e.is_hands_free
                    && (e.id.to_lowercase().contains("sony")
                        || e.id.to_lowercase().contains("1000xm"))
            }),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_render_endpoints_runs_safely() {
        // Test passes without panicking in CI or local environment
        let result = AudioEndpointSanitizer::enumerate_render_endpoints();
        assert!(result.is_ok());
    }
}
