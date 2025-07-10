use crate::errors::FakitError;
use std::io::Write;

#[inline]
pub fn wrap_fasta(seq_slice: &[u8], line_width: usize) -> Result<Vec<u8>, FakitError> {
    let seq_len = seq_slice.len();
    let mut seq_new: Vec<&[u8]> = vec![];
    let mut index = 0usize;

    loop {
        index += line_width;
        let start = index - line_width;
        if index < seq_len {
            let window = &seq_slice[start..index];
            seq_new.push(window);
            seq_new.push("\n".as_bytes());
        } else {
            index = seq_len;
            let window = &seq_slice[start..index];
            seq_new.push(window);
        }
        // line end
        if index == seq_len {
            break;
        }
    }
    let seq_wrap = seq_new.concat();

    Ok(seq_wrap)
}

pub fn write_record<W>(
    writer: &mut W,
    id: &[u8],
    seq: &[u8],
    line_width: usize,
) -> Result<(), FakitError>
where
    W: Write + Send,
{
    writer.write_all(b">")?;
    writer.write_all(id)?;
    writer.write_all(b"\n")?;
    match line_width {
        0 => writer.write_all(seq)?,
        _ => line_wrap(seq, line_width, writer)?,
    }
    writer.write_all(b"\n")?;
    Ok(())
}

#[inline]
pub fn line_wrap(
    seq_slice: &[u8],
    line_width: usize,
    writer: &mut impl std::io::Write,
) -> Result<(), FakitError> {
    let seq_len = seq_slice.len();
    let mut index = 0usize;

    loop {
        index += line_width;
        let start = index - line_width;
        if index < seq_len {
            let window = &seq_slice[start..index];
            writer.write_all(window)?;
            writer.write_all(b"\n")?;
        } else {
            index = seq_len;
            let window = &seq_slice[start..index];
            writer.write_all(window)?;
        }
        // line end
        if index == seq_len {
            break;
        }
    }

    Ok(())
}
