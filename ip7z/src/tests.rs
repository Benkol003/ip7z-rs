use std::error::Error;

use strum::IntoEnumIterator;

use crate::{ICoder::MethodPropID, codecs::Codecs, ffi::Z7, win_ffi::{HRESULT, PROPVARIANT}};

#[test]
fn check_codecs() -> Result<(), Box<dyn Error>>{
    unsafe {
        let z7 = Z7::new()?;
        let mut num: u32 = 0;
        HRESULT::from(z7.0.GetNumberOfMethods(&mut num as *mut _)).ok()?;
        println!("num coders: {}",num);

        let mut supported_codecs: Vec<Codecs> = Vec::new();
        let mut missing_encoders: Vec<Codecs> = Vec::new();
        let mut missing_decoders: Vec<Codecs> = Vec::new();

        for i in 0..num {
            let mut id = PROPVARIANT::default();
            z7.GetMethodProperty(i, MethodPropID::kID, &mut id);
            let id = u64::try_from(id)?;
            let id = Codecs::try_from(id).map_err(|e| format!("Unknown codec: {}",e)).unwrap();

            if id.supports_encoder() {
                let mut has_encoder = PROPVARIANT::default();
                z7.GetMethodProperty(i, MethodPropID::kEncoderIsAssigned, &mut has_encoder);
                let has_encoder = bool::try_from(has_encoder).unwrap();
                if !has_encoder {
                    missing_encoders.push(id);
                }
            }

            let mut has_decoder = PROPVARIANT::default();
            z7.GetMethodProperty(i,MethodPropID::kDecoderIsAssigned, &mut has_decoder);
            let has_decoder = bool::try_from(has_decoder).unwrap();
            if !has_decoder {
                missing_decoders.push(id);
            }

            supported_codecs.push(id);
        }

        let missing_codecs: Vec<_> = Codecs::iter().filter(|x| !supported_codecs.contains(x)).collect();

        let mut missing = false;
        let mut err = String::new();

        if missing_codecs.len() != 0 {
            missing=true;
            err.push_str(format!("missing codecs: {:?}\n",missing_codecs).as_str());
        }
        if missing_encoders.len() !=0 {
            missing=true;
            err.push_str(format!("missing encoders for codecs: {:?}\n",missing_encoders).as_str());
        }
        if missing_decoders.len() !=0 {
            missing=true;
            err.push_str(format!("missing decoders for codecs: {:?}\n",missing_decoders).as_str());
        }

        if missing {
            panic!("{}",err);
        }

        Ok(())
    }
}