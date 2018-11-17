use std::fmt;

struct Rectangle
{
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Complex
{
    real: i32,
    image: i32,
}

impl fmt::Display for Complex
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} + {}i", self.real, self.image)
    }
}

impl fmt::Display for Rectangle
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main()
{
    // 通常情况下， `{}` 会被任意变量内容所替换。
    // 值内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31自动成为 I32 类型。
    // 你可以添加后缀来改变 31 的原来类型。

    // 下面有多种可选形式。
    // 可以使用的位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用赋值语句。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 特殊的格式实现可以在后面加上 `:` 符号。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出"     1"，5个空格后面连着1。
    println!("{number:>width$}", number=1, width=6);

    // 你可以对数字左边位数上补0。下面语句输出"000001"。
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 改正 ^ 补上漏掉的参数： "James"

    // 创建一个包含` I32 `类型结构体(structure)。命名为 `Structure`。
    

    // 但是像结构体这样自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // 改了一下就能跑了
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {}", rect1);
    // 改正 ^ 注释掉此行。
    let pi = 3.1415926535;
    // let pi = format!("{:.*}", 2, pi);
    println!("{:.*}", 2, pi);

    let c = Complex { real: 3, image: 4 };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);


}