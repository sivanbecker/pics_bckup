use std::fs;
use walkdir::WalkDir;

struct LocalDir {
    top: bool,
    to_album: bool,
    path: String,
    sub_amount: u8,
}

fn main() {
    //const P1: &str = "/home/sn/our_pics/oldE/gDrive/picturesGdrive/traveling1/";
    //const P2: &str = "/home/sn/our_pics/oldE/gDrive/picturesGdrive";
    const P3: &str =
        "/home/sn/our_pics/oldE/gDrive/picturesGdrive/traveling1/Holland & Belgume Apr. 2009/";
    let usertop = LocalDir {
        top: true,
        to_album: false,
        path: String::from(P3),
        sub_amount: 0u8,
    };

    //let paths = fs::read_dir(&usertop.path).unwrap();

    //for path in paths {
    //    println!("Name: {}", path.unwrap().path().display())
    //}
    println!("TOP TREE {}", usertop.path);
    // recursive_walk(usertop);
    let mut v: Vec<LocalDir> = Vec::new();
    v.push(usertop);
    recursive_walk(&v);
    for i in &v {
        println!("{} {}", i.path, i.sub_amount);
    }
}

// fn traverse_dir(mut topdir: LocalDir) -> Result<()> {
//     for entry in fs::read_dir(topdir.path) {
//         let entry = entry?;
//         let path = entry.path();

//         let metadata = fs::metadata(&path)?;
//         if entry.path().is_dir() {
//             println!("{}", entry.path().display());
//         }
//     }
//     Ok(())
// }
fn recursive_walk(ve: &Vec<LocalDir>) -> &Vec<LocalDir> {
    for entry in WalkDir::new(&ve[1].path).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        if entry.path().is_dir() {
            ve.push(LocalDir {
                top: false,
                to_album: false,
                path: String::from(entry.file_name().to_string_lossy()),
                sub_amount: 0,
            })
        } else {
            ve[1].sub_amount += 1;
        }
        if ve.len() > 5 {
            break;
        }
    }
    ve
}

//struct LocalAlbumDir {
//    path: String,
//    pics_amount: u16,
//}
//struct TopTree {
//   path: String,
//   sub_dirs: Vec<LocalAlbumDir>,
//}
