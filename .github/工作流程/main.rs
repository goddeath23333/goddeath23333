use std::fs;
fn name() -> std::io::Result {
    fs::create_dir("./mydir")?;
    fs::create_dir("./mydir/test")?;
    let path = path::new("./mypath/a.txt");
    let parent = path.parent().unwrap();
    let fname = path.flie_stem().unwrap();
    OK(())
}
