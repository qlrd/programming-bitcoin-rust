use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl Signature {
    /// create a signature from BigUint
    pub fn from_biguint(r: BigUint, s: BigUint) -> Result<Self, String> {
        if r.to_bytes_be().len() == 32 {
            let r_vec = <[u8; 32]>::try_from(r.to_bytes_be()).unwrap().to_vec();
            let s_vec = <[u8; 32]>::try_from(s.to_bytes_be()).unwrap().to_vec();
            Ok(Signature::new(r_vec, s_vec).unwrap())
        } else {
            let r_vec = <[u8; 33]>::try_from(r.to_bytes_be()).unwrap().to_vec();
            let s_vec = <[u8; 32]>::try_from(s.to_bytes_be()).unwrap().to_vec();
            Ok(Signature::new(r_vec, s_vec).unwrap())
        }
    }

    /// Create a Signature from two vectors.
    /// The `r` value can be 32 or 33 bytes; the `s`
    /// value should be 32 bytes
    pub fn new(r: Vec<u8>, s: Vec<u8>) -> Result<Self, String> {
        println!("len == {}: {:?}", r.len(), r);
        if r.len() != 32 && r.len() != 33 {
            return Err("R value should have 32 or 33 bytes length".to_string());
        }

        Ok(Self { r, s })
    }

    /// Serialize the current Signature struct to bitcoin's DER format
    pub fn der(&self) -> Result<Vec<u8>, String> {
        // start with 0x30 byte, equivalent 48u8
        let mut serialized = vec![48u8];

        let serialize = |element: &Vec<u8>| -> Result<Vec<u8>, String> {
            if element.is_empty() {
                return Err("Signature element cannot be empty.".to_string());
            }

            // Append the 0x02 marker
            let mut res = vec![2u8];

            // Prepend 0x00 if the first byte is >= 0x80 (MSB is set)
            if element[0] >= 128u8 {
                res.push((element.len() + 1) as u8);
                res.push(0u8);
            } else {
                res.push(element.len() as u8);
            }

            // Append the element itself
            res.extend_from_slice(element.as_slice());
            Ok(res)
        };

        let r = serialize(&self.r).map_err(|e| format!("Error serializing 'r': {}", e))?;

        let s = serialize(&self.s).map_err(|e| format!("Error serializing 's': {}", e))?;

        let len = r.len() + s.len();

        if len > 255 {
            return Err(format!(
                "Total serialized length exceeds maximum allowable value: {} bytes.",
                len
            ));
        }

        serialized.extend_from_slice(&[len as u8]);
        serialized.extend_from_slice(&r);
        serialized.extend_from_slice(&s);
        Ok(serialized)
    }
}
