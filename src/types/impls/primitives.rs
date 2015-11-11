use expression::{Expression, AsExpression};
use expression::bound::Bound;
use std::error::Error;
use std::io::Write;
use super::option::UnexpectedNullError;
use types::{NativeSqlType, FromSql, ToSql, IsNull};
use {Queriable, types};

macro_rules! primitive_impls {
    ($($Source:ident -> $Target:ty),+,) => {
        $(
            impl NativeSqlType for types::$Source {}

            impl Queriable<types::$Source> for $Target {
                type Row = Self;

                fn build(row: Self::Row) -> Self {
                    row
                }
            }

            impl AsExpression<types::$Source> for $Target {
                type Expression = Bound<types::$Source, Self>;

                fn as_expression(self) -> Self::Expression {
                    Bound::new(self)
                }
            }

            impl AsExpression<types::Nullable<types::$Source>> for $Target {
                type Expression = <Self as AsExpression<types::$Source>>::Expression;

                fn as_expression(self) -> Self::Expression {
                    AsExpression::<types::$Source>::as_expression(self)
                }
            }
        )+
    }
}

primitive_impls! {
    Bool -> bool,

    SmallSerial -> i16,
    Serial -> i32,
    BigSerial -> i64,

    SmallInt -> i16,
    Integer -> i32,
    BigInt -> i64,

    Float -> f32,
    Double -> f64,

    VarChar -> String,
    Text -> String,

    Binary -> Vec<u8>,
}

impl FromSql<types::Bool> for bool {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let bytes = not_none!(bytes);
        Ok(bytes[0] != 0)
    }
}

impl ToSql<types::Bool> for bool {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        let write_result = if *self {
            out.write_all(&[1])
        } else {
            out.write_all(&[0])
        };
        write_result
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl FromSql<types::VarChar> for String {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let bytes = not_none!(bytes);
        String::from_utf8(bytes.into()).map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl ToSql<types::VarChar> for String {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<'a> ToSql<types::VarChar> for &'a str {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<'a> AsExpression<types::VarChar> for &'a str {
    type Expression = Bound<types::VarChar, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<types::Nullable<types::VarChar>> for &'a str {
    type Expression = <Self as AsExpression<types::VarChar>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<types::VarChar>::as_expression(self)
    }
}

impl<'a> AsExpression<types::Text> for &'a str {
    type Expression = Bound<types::Text, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<types::Nullable<types::Text>> for &'a str {
    type Expression = <Self as AsExpression<types::Text>>::Expression;

    fn as_expression(self) -> Self::Expression {
        AsExpression::<types::Text>::as_expression(self)
    }
}

impl FromSql<types::Text> for String {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        <Self as FromSql<types::VarChar>>::from_sql(bytes)
    }
}

impl ToSql<types::Text> for String {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        ToSql::<types::VarChar>::to_sql(self, out)
    }
}

impl<'a> ToSql<types::Text> for &'a str {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        ToSql::<types::VarChar>::to_sql(self, out)
    }
}

impl FromSql<types::Binary> for Vec<u8> {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        Ok(not_none!(bytes).into())
    }
}

impl ToSql<types::Binary> for Vec<u8> {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(&self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<'a> ToSql<types::Binary> for &'a [u8] {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

#[test]
fn bool_to_sql() {
    let mut bytes = vec![];
    true.to_sql(&mut bytes).unwrap();
    false.to_sql(&mut bytes).unwrap();
    assert_eq!(bytes, vec![1u8, 0u8]);
}
