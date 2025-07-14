use crate::errors::FakitError;
use std::io::Write;

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
