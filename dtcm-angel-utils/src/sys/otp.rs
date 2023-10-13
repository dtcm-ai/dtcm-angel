use totp_rs::{Algorithm, Secret, TOTP};

use crate::Result;

/// Generates the current OTP
pub fn otp<O>(otp_token: O) -> Result<String>
where
    O: Into<String>,
{
    debug!("Fetching current OTP");
    let secret = Secret::Encoded(otp_token.into());

    let secret_bytes = secret.to_bytes().map_err(|e| e.to_string())?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret_bytes)?;

    totp.generate_current().map_err(|e| {
        error!("OTP fetch failed: {e}");
        e.into()
    })
}
