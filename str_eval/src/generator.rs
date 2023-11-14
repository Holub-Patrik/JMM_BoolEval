fn _bool_table(size:u32, true_val_repr:u128) -> () {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut w = std::io::BufWriter::new(lock);

    let mut expr_val:u128 = 0;
    let complete_add_value:u128 = !true_val_repr;

    for _ in 0 .. 2_u128.pow(size.try_into().unwrap()) {
        if complete_add_value | expr_val > complete_add_value {
            expr_val+=1;
            continue;
        }
        let actual_val = (complete_add_value & expr_val)+true_val_repr;
        //println!("actual val: {} and expr_val = {}",actual_val, expr_val);
        for i in 0 .. size {
            let curr:u128 = actual_val >> i&1;
            write!(&mut w, "{} ", curr.to_string()).unwrap();
        }
        writeln!(&mut w, "").unwrap();
        expr_val+=1;
        }
    println!("");
}
