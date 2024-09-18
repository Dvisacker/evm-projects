use super::{
    DbCurvePool, DbErc4626Vault, DbUniV2Pool, DbUniV3Pool, NewDbCurvePool, NewDbErc4626Vault,
    NewDbUniV2Pool, NewDbUniV3Pool,
};
pub enum NewDbPool {
    UniV2(NewDbUniV2Pool),
    UniV3(NewDbUniV3Pool),
    ERC4626Vault(NewDbErc4626Vault),
    Curve(NewDbCurvePool),
}

pub enum DbPool {
    UniV2(DbUniV2Pool),
    UniV3(DbUniV3Pool),
    ERC4626Vault(DbErc4626Vault),
    Curve(DbCurvePool),
}
