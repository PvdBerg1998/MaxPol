use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::Read;

const MAX_N: usize = 4;
const MAX_L: usize = 100;

fn main() -> Result<()> {
    fs::remove_dir_all("maxpol_coefficients")?;

    let db = sled::Config::default()
        .path("maxpol_coefficients")
        .use_compression(true)
        .mode(sled::Mode::LowSpace)
        .open()?;

    let mut filename = String::new();
    let mut buf = String::new();
    let mut coeff = [0f64; 2 * MAX_L + 1];

    for n in 1..=MAX_N {
        for l in 1..=MAX_L {
            for p in n..=2 * l {
                filename.clear();
                write!(&mut filename, "../maxpol_calc/maxpol_n{}_l{}_P{}", n, l, p).unwrap();

                let mut file = fs::File::open(&filename)?;
                buf.clear();
                file.read_to_string(&mut buf)?;

                for (line, c) in buf.lines().zip(coeff.iter_mut()) {
                    *c = fast_float::parse(line).expect("parse error");
                }

                let key = bincode::serialize(&(n, l, p))?;
                let value = bincode::serialize(&coeff[0..2 * l + 1])?;

                db.insert(key, value)?;
            }
        }
    }

    db.flush()?;

    Ok(())
}
