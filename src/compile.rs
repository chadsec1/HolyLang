use crate::error::HolyError;

pub fn compile(rcode: &str) -> Result<(), HolyError> {
    println!("hm: {rcode:?}");

    Ok(())
}
