use std::path::PathBuf;

use anyhow::{Result, anyhow, Context};



fn error_me(throw: bool) -> Result<()> {

    if throw {
        return Err(anyhow!("this should never be true"));
    }

    std::fs::read(PathBuf::from("/foo"))

    return Ok(());
}




fn main() -> Result<(),u_size> {

    error_me(false)?;

    if error_me(true).is_ok(){

    }



    let value = match error_me(false) {
        Err(e) => return Err(e),
        Ok(v)=>v,
        
    };


    return Ok(());

}
