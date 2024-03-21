use anyhow::{Ok,Result};

#[inline]
pub fn wrap_fasta(
    seq_slice: &[u8],
    line_width: usize,
) -> Result<Vec<u8>> {
    
    let seq_len = seq_slice.len();
    let mut seq_new: Vec<&[u8]> = vec![];
    let mut index = 0usize;

    loop {
        index += line_width;
        let start = index - line_width;
        if index <= seq_len {
            let window = &seq_slice[start..index];
            seq_new.push(window);
            seq_new.push("\n".as_bytes());
        } else {
            index = seq_len;
            let window = &seq_slice[start..index];
            seq_new.push(window);
        }
        // line end
        if index == seq_len { break; }
    }
    let seq_wrap = seq_new.concat();
    
    Ok(seq_wrap)
}