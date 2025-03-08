use core::blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    
    bc.add_block("First Block".into());
    bc.add_block("Second Block".into());

    bc.print_chain();
}
