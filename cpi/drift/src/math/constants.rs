use solana_program::native_token::LAMPORTS_PER_SOL; // expo 9
pub const LAMPORTS_PER_SOL_U64: u64 = LAMPORTS_PER_SOL;
pub const LAMPORTS_PER_SOL_I64: i64 = LAMPORTS_PER_SOL as i64;

// SPOT MARKET CONSTANTS
pub const QUOTE_SPOT_MARKET_INDEX: u16 = 0;

// USER ACCOUNT CONSTANTS
pub const MAX_SPOT_POSITIONS: u8 = 8;
pub const MAX_PERP_POSITIONS: u8 = 8;
pub const MAX_OPEN_ORDERS: u8 = 32;

// PRECISIONS
pub const AMM_RESERVE_PRECISION: u128 = 1_000_000_000; //expo = -9;
pub const AMM_RESERVE_PRECISION_I128: i128 = (AMM_RESERVE_PRECISION) as i128;
pub const BASE_PRECISION: u128 = AMM_RESERVE_PRECISION; //expo = -9;
pub const BASE_PRECISION_I128: i128 = AMM_RESERVE_PRECISION_I128;
pub const BASE_PRECISION_U64: u64 = AMM_RESERVE_PRECISION as u64; //expo = -9;
pub const BASE_PRECISION_I64: i64 = AMM_RESERVE_PRECISION_I128 as i64; //expo = -9;
pub const PERP_DECIMALS: u32 = 9;

pub const PRICE_PRECISION: u128 = 1_000_000; //expo = -6;
pub const PRICE_PRECISION_I128: i128 = PRICE_PRECISION as i128;
pub const PRICE_PRECISION_U64: u64 = 1_000_000; //expo = -6;
pub const PRICE_PRECISION_I64: i64 = 1_000_000; //expo = -6;

pub const PEG_PRECISION: u128 = 1_000_000; //expo = -6
pub const PEG_PRECISION_I128: i128 = PEG_PRECISION as i128; //expo = -6

pub const QUOTE_PRECISION: u128 = 1_000_000; // expo = -6
pub const QUOTE_PRECISION_I128: i128 = 1_000_000; // expo = -6
pub const QUOTE_PRECISION_I64: i64 = 1_000_000; // expo = -6
pub const QUOTE_PRECISION_U64: u64 = 1_000_000; // expo = -6

pub const FUNDING_RATE_BUFFER: u128 = 1_000; // expo = -3
pub const FUNDING_RATE_BUFFER_I128: i128 = FUNDING_RATE_BUFFER as i128; // expo = -3

pub const MARGIN_PRECISION: u32 = 10_000; // expo = -4
pub const MARGIN_PRECISION_U128: u128 = 10_000; // expo = -4
pub const SPOT_WEIGHT_PRECISION: u32 = MARGIN_PRECISION; // expo = -4
pub const SPOT_WEIGHT_PRECISION_U128: u128 = SPOT_WEIGHT_PRECISION as u128; // expo = -4
pub const SPOT_WEIGHT_PRECISION_I128: i128 = SPOT_WEIGHT_PRECISION as i128; // expo = -4

pub const LIQUIDATION_PCT_PRECISION: u128 = 10_000;

pub const SPOT_BALANCE_PRECISION: u128 = 1_000_000_000; // expo = -9
pub const SPOT_BALANCE_PRECISION_U64: u64 = 1_000_000_000; // expo = -9
pub const SPOT_CUMULATIVE_INTEREST_PRECISION: u128 = 10_000_000_000; // expo = -10

pub const PERCENTAGE_PRECISION: u128 = 1_000_000; // expo -6 (represents 100%)
pub const PERCENTAGE_PRECISION_I128: i128 = PERCENTAGE_PRECISION as i128;
pub const PERCENTAGE_PRECISION_U64: u64 = PERCENTAGE_PRECISION as u64;
pub const TEN_BPS: i128 = PERCENTAGE_PRECISION_I128 / 1000;
pub const TEN_BPS_I64: i64 = TEN_BPS as i64;
pub const TWO_PT_TWO_PCT: i128 = 22_000;

pub const BID_ASK_SPREAD_PRECISION: u64 = PERCENTAGE_PRECISION as u64; // expo = -6
pub const BID_ASK_SPREAD_PRECISION_I64: i64 = (BID_ASK_SPREAD_PRECISION) as i64;
pub const BID_ASK_SPREAD_PRECISION_U128: u128 = BID_ASK_SPREAD_PRECISION as u128; // expo = -6
pub const BID_ASK_SPREAD_PRECISION_I128: i128 = BID_ASK_SPREAD_PRECISION as i128; // expo = -6

pub const CONCENTRATION_PRECISION: u128 = PERCENTAGE_PRECISION; // expo 6
pub const IF_FACTOR_PRECISION: u128 = PERCENTAGE_PRECISION; // expo 6

pub const SPOT_UTILIZATION_PRECISION: u128 = PERCENTAGE_PRECISION; // expo = -6
pub const SPOT_UTILIZATION_PRECISION_U32: u32 = PERCENTAGE_PRECISION as u32; // expo = -6
pub const SPOT_RATE_PRECISION: u128 = PERCENTAGE_PRECISION; // expo = -6
pub const SPOT_RATE_PRECISION_U32: u32 = PERCENTAGE_PRECISION as u32; // expo = -6
pub const LIQUIDATION_FEE_PRECISION: u32 = PERCENTAGE_PRECISION as u32; // expo = -6
pub const LIQUIDATION_FEE_PRECISION_U128: u128 = LIQUIDATION_FEE_PRECISION as u128; // expo = -6
pub const SPOT_IMF_PRECISION: u32 = PERCENTAGE_PRECISION as u32; // expo = -6
pub const SPOT_IMF_PRECISION_U128: u128 = SPOT_IMF_PRECISION as u128; // expo = -6

// FORMULAIC REPEG / K
pub const K_BPS_UPDATE_SCALE: i128 = PERCENTAGE_PRECISION_I128;
pub const PEG_BPS_UPDATE_SCALE: u128 = PERCENTAGE_PRECISION; // expo = -6 (represents 100%)

// PRECISION CONVERSIONS
pub const PRICE_TO_PEG_PRECISION_RATIO: u128 = PRICE_PRECISION / PEG_PRECISION; // expo: 1 (Delete if we keep peg/price as 1e6)
pub const AMM_TO_QUOTE_PRECISION_RATIO: u128 = AMM_RESERVE_PRECISION / QUOTE_PRECISION; // expo: 3
pub const AMM_TO_QUOTE_PRECISION_RATIO_I128: i128 =
    (AMM_RESERVE_PRECISION / QUOTE_PRECISION) as i128; // expo: 3
pub const AMM_TIMES_PEG_TO_QUOTE_PRECISION_RATIO: u128 =
    AMM_RESERVE_PRECISION * PEG_PRECISION / QUOTE_PRECISION; // expo: 9
pub const QUOTE_TO_BASE_AMT_FUNDING_PRECISION: i128 =
    (AMM_RESERVE_PRECISION_I128 * FUNDING_RATE_PRECISION_I128 / QUOTE_PRECISION_I128) as i128; // expo: 12
pub const PRICE_TO_QUOTE_PRECISION_RATIO: u128 = PRICE_PRECISION / QUOTE_PRECISION; // expo: 1
pub const PRICE_TIMES_AMM_TO_QUOTE_PRECISION_RATIO: u128 =
    PRICE_PRECISION * AMM_TO_QUOTE_PRECISION_RATIO; // expo 9
pub const LIQUIDATION_FEE_TO_MARGIN_PRECISION_RATIO: u32 = // expo 2
    LIQUIDATION_FEE_PRECISION / MARGIN_PRECISION;
pub const LIQUIDATION_FEE_TO_MARGIN_PRECISION_RATIO_U128: u128 = // expo 2
    LIQUIDATION_FEE_TO_MARGIN_PRECISION_RATIO as u128;
pub const FUNDING_RATE_TO_QUOTE_PRECISION_PRECISION_RATIO: u128 = // expo 3
    FUNDING_RATE_PRECISION / QUOTE_PRECISION;
pub const FUNDING_RATE_PRECISION: u128 = PRICE_PRECISION * FUNDING_RATE_BUFFER; // expo: 9
pub const FUNDING_RATE_PRECISION_I128: i128 = PRICE_PRECISION_I128 * FUNDING_RATE_BUFFER_I128; // expo: 9
pub const FUNDING_RATE_PRECISION_I64: i64 = FUNDING_RATE_PRECISION_I128 as i64; // expo: 9

pub const AMM_TIMES_PEG_TO_QUOTE_PRECISION_RATIO_I128: i128 =
    AMM_TIMES_PEG_TO_QUOTE_PRECISION_RATIO as i128;
pub const PRICE_TIMES_AMM_TO_QUOTE_PRECISION_RATIO_I128: i128 =
    PRICE_TIMES_AMM_TO_QUOTE_PRECISION_RATIO as i128; // expo 9

// FEE REBATES
pub const SHARE_OF_FEES_ALLOCATED_TO_DRIFT_NUMERATOR: u128 = 1;
pub const SHARE_OF_FEES_ALLOCATED_TO_DRIFT_DENOMINATOR: u128 = 2;

pub const SHARE_OF_IF_ESCROW_ALLOCATED_TO_PROTOCOL_NUMERATOR: u128 = 1;
pub const SHARE_OF_IF_ESCROW_ALLOCATED_TO_PROTOCOL_DENOMINATOR: u128 = 2;

pub const SHARE_OF_REVENUE_ALLOCATED_TO_INSURANCE_FUND_VAULT_NUMERATOR: u128 = 1;
pub const SHARE_OF_REVENUE_ALLOCATED_TO_INSURANCE_FUND_VAULT_DENOMINATOR: u128 = 1;

// TIME PERIODS
pub const ONE_MINUTE: i128 = 60_i128;
pub const FIVE_MINUTE: i128 = (60 * 5) as i128;
pub const ONE_HOUR: i64 = 3600;
pub const ONE_HOUR_I128: i128 = ONE_HOUR as i128;
pub const TWENTY_FOUR_HOUR: i64 = 3600 * 24;
pub const THIRTEEN_DAY: i64 = TWENTY_FOUR_HOUR * 13; // IF unstake default
pub const EPOCH_DURATION: i64 = TWENTY_FOUR_HOUR * 28;
pub const THIRTY_DAY: i64 = TWENTY_FOUR_HOUR * 30;
pub const THIRTY_DAY_I128: i128 = (TWENTY_FOUR_HOUR * 30) as i128;
pub const ONE_YEAR: u128 = 31536000;

// QUOTE AMOUNTS
pub const ONE_HUNDRED_MILLION_QUOTE: u64 = 100_000_000_u64 * QUOTE_PRECISION_U64;
pub const FIFTY_MILLION_QUOTE: u64 = 50_000_000_u64 * QUOTE_PRECISION_U64;
pub const TEN_MILLION_QUOTE: u64 = 10_000_000_u64 * QUOTE_PRECISION_U64;
pub const FIVE_MILLION_QUOTE: u64 = 10_000_000_u64 * QUOTE_PRECISION_U64;
pub const ONE_MILLION_QUOTE: u64 = 1_000_000_u64 * QUOTE_PRECISION_U64;
pub const TWO_HUNDRED_FIFTY_THOUSAND_QUOTE: u64 = 250_000_u64 * QUOTE_PRECISION_U64;
pub const ONE_HUNDRED_THOUSAND_QUOTE: u64 = 100_000_u64 * QUOTE_PRECISION_U64;
pub const TWENTY_FIVE_THOUSAND_QUOTE: u64 = 25_000_u64 * QUOTE_PRECISION_U64;
pub const TEN_THOUSAND_QUOTE: u64 = 10_000_u64 * QUOTE_PRECISION_U64;
pub const ONE_THOUSAND_QUOTE: u64 = 1_000_u64 * QUOTE_PRECISION_U64;
pub const TWO_HUNDRED_FIFTY_QUOTE: u64 = 250_u64 * QUOTE_PRECISION_U64;

// INSURANCE TIERS
pub const INSURANCE_A_MAX: u64 = ONE_HUNDRED_MILLION_QUOTE as u64;
pub const INSURANCE_B_MAX: u64 = ONE_HUNDRED_THOUSAND_QUOTE as u64;
pub const INSURANCE_C_MAX: u64 = TWENTY_FIVE_THOUSAND_QUOTE as u64;
pub const INSURANCE_SPECULATIVE_MAX: u64 = 0;

// QUOTE THRESHOLDS
pub const FEE_POOL_TO_REVENUE_POOL_THRESHOLD: u128 = TWO_HUNDRED_FIFTY_QUOTE as u128;

// FEES
pub const ONE_BPS_DENOMINATOR: u32 = 10000;
pub const MAX_REFERRER_REWARD_EPOCH_UPPER_BOUND: u64 = (4000 * QUOTE_PRECISION) as u64;
pub const LP_FEE_SLICE_NUMERATOR: u128 = 8;
pub const LP_FEE_SLICE_DENOMINATOR: u128 = 10;
pub const FEE_DENOMINATOR: u32 = 10 * ONE_BPS_DENOMINATOR;
pub const FEE_PERCENTAGE_DENOMINATOR: u32 = 100;
pub const OPEN_ORDER_MARGIN_REQUIREMENT: u128 = QUOTE_PRECISION / 100;
pub const FEE_ADJUSTMENT_MAX: u64 = 100;

// PRICE AMOUNTS
pub const HUNDRENTH_OF_CENT: u128 = PRICE_PRECISION / 10_000; //.0001

// CONSTRAINTS
pub const MAX_K_BPS_INCREASE: i128 = TEN_BPS;
pub const MAX_K_BPS_DECREASE: i128 = TWO_PT_TWO_PCT;
pub const MAX_UPDATE_K_PRICE_CHANGE: u128 = HUNDRENTH_OF_CENT;
pub const MAX_SQRT_K: u128 = 1000000000000000000000; // 1e21 (count 'em!)
pub const MAX_BASE_ASSET_AMOUNT_WITH_AMM: u128 = 100000000000000000; // 1e17 (count 'em!)

pub const MAX_PEG_BPS_INCREASE: u128 = TEN_BPS as u128; // 10 bps increase
pub const MAX_PEG_BPS_DECREASE: u128 = TEN_BPS as u128; // 10 bps decrease

pub const MAX_APR_PER_REVENUE_SETTLE_TO_INSURANCE_FUND_VAULT: u64 = 10 * PERCENTAGE_PRECISION_U64; // 1000% APR

pub const MAX_CONCENTRATION_COEFFICIENT: u128 = 1_414_200;
pub const MAX_LIQUIDATION_SLIPPAGE: i128 = 10_000; // expo = -2
pub const MAX_LIQUIDATION_SLIPPAGE_U128: u128 = 10_000; // expo = -2
pub const MAX_MARK_TWAP_DIVERGENCE: u128 = 500_000; // expo = -3

pub const MAX_MARGIN_RATIO: u32 = MARGIN_PRECISION as u32; // 1x leverage
pub const MIN_MARGIN_RATIO: u32 = MARGIN_PRECISION as u32 / 50; // 50x leverage

pub const MAX_BID_ASK_INVENTORY_SKEW_FACTOR: u64 = 10 * BID_ASK_SPREAD_PRECISION;

pub const MAX_POSITIVE_UPNL_FOR_INITIAL_MARGIN: i128 = 100 * QUOTE_PRECISION_I128; // max upnl for initial margin calc
pub const DEFAULT_MAX_TWAP_UPDATE_PRICE_BAND_DENOMINATOR: i64 = 3; // '3' here means clamp new data point to 33% (1/3) divergence from current twap (if twap > 0)

// DEFAULTS
pub const DEFAULT_REVENUE_SINCE_LAST_FUNDING_SPREAD_RETREAT: i64 = -25 * QUOTE_PRECISION_I64; //$25 loss
pub const DEFAULT_LARGE_BID_ASK_FACTOR: u64 = 10 * BID_ASK_SPREAD_PRECISION;
pub const DEFAULT_LIQUIDATION_MARGIN_BUFFER_RATIO: u32 = (MARGIN_PRECISION as u32) / 50; // 2%
pub const DEFAULT_BASE_ASSET_AMOUNT_STEP_SIZE: u64 = BASE_PRECISION_U64 / 10000; // 1e-4;
pub const DEFAULT_QUOTE_ASSET_AMOUNT_TICK_SIZE: u64 =
    PRICE_PRECISION_U64 / DEFAULT_BASE_ASSET_AMOUNT_STEP_SIZE; // 1e-2

// FUNDING
pub const FUNDING_RATE_OFFSET_DENOMINATOR: i64 = 5000; // 5000 => 7.3% annualized rate for hourly funding

// ORDERS
pub const AUCTION_DERIVE_PRICE_FRACTION: i64 = 200;

// WITHDRAWS
pub const SPOT_MARKET_TOKEN_TWAP_WINDOW: i64 = TWENTY_FOUR_HOUR;