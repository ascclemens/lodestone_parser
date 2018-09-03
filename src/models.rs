macro_rules! ffxiv_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident => $str_repr:expr),+$(,)? }) => {
      $(#[$meta])*
      #[derive(Debug, Serialize, Deserialize)]
      pub enum $name {
        $($variant,)+
      }

      impl $name {
        pub fn parse(s: &str) -> Option<Self> {
          let res = match s {
            $($str_repr => $name::$variant,)+
            _ => return None,
          };
          Some(res)
        }

        pub fn name(&self) -> &str {
          match *self {
            $($name::$variant => $str_repr,)+
          }
        }
      }
    }
}

pub mod character;
pub mod free_company;
pub mod search;

ffxiv_enum!(
  #[derive(PartialEq, Eq, PartialOrd, Ord)]
  GrandCompany {
    Flames => "Immortal Flames",
    Maelstrom => "Maelstrom",
    TwinAdders => "Order of the Twin Adder",
  }
);
