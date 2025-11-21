pub fn annotate(garden: &[&str]) -> Vec<String> {
    assert!(
        garden.iter().all(|r| r.is_ascii()),
        "garden contains non-ASCII characters"
    );
    let mut annotate_garden :Vec<String>= Vec::new();

    let rows = garden.iter().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();
    for (i,row) in rows.iter().enumerate() {
        let mut _row = Vec::<u8>::with_capacity(row.len());

        for (j, ch) in row.iter().enumerate() {
            if *ch == b'*' {
                _row.push(b'*');
            }
            else {
                let mut flower_count = 0usize;
                for dx in -1isize..=1 {
                    for dy in -1isize..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                     let ni = i as isize + dx;
                        let nj = j as isize + dy;
                        if ni >= 0 && (ni as usize) < rows.len() {
                            let neigh_row = rows[ni as usize];
                            if nj >= 0 && (nj as usize) < neigh_row.len() {
                                if neigh_row[nj as usize] == b'*' {
                                    flower_count += 1;
                                }
                            }
                        }
                    }
                }

                if flower_count > 0 {
                    _row.push(b'0' + (flower_count as u8));
                } else {
                    _row.push(b' ')
                }
            }
        }
        annotate_garden.push(String::from_utf8(_row).unwrap());

    }
    annotate_garden
    
}