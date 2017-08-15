# [boolfuck]

A simple boolfuck interpreter made in rust

# Examples

 - "Hello, World!"

```
extern crate boolfuck;
use boolfuck::Boolfuck;

fn main() {
  let program = Boolfuck::new(";;;+;+;;+;+;
                              +;+;+;+;;+;;+;
                              ;;+;;+;+;;+;
                              ;;+;;+;+;;+;
                              +;;;;+;+;;+;
                              ;;+;;+;+;+;;
                              ;;;;;+;+;;
                              +;;;+;+;;;+;
                              +;;;;+;+;;+;
                              ;+;+;;+;;;+;
                              ;;+;;+;+;;+;
                              ;;+;+;;+;;+;
                              +;+;;;;+;+;;
                              ;+;+;+;");
                              
  program.run(false);
}
```

For a general explanation of `boolfuck` visit the [*offical website*].

[*offical website*]:http://samuelhughes.com/boof/
[boolfuck]:https://crates.io/crates/boolfuck
