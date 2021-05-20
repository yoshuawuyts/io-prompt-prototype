fn main() -> std::io::Result<()> {
    use io_prompt_prototype::prompt;

    let num = prompt!("What's your favorite number?: ");
    println!("Oh, cool: {}!", num);
    Ok(())
}
