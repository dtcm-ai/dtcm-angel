use std::fmt::Display;

use self::EndPoint::*;

// Root URL for API connect platform
const ROOT_URL: &str = "https://apiconnect.angelbroking.com";

// Websocket URL
const WS_URL: &str = "ws://smartapisocket.angelone.in/smart-stream";

/// URL to download the instrument list
pub const INSTRUMENT_URL: &str =
    "https://margincalculator.angelbroking.com/OpenAPI_File/files/OpenAPIScripMaster.json";

// Publisher login URL
const PUBLISHER_LOGIN: &str = "https://smartapi.angelbroking.com/publisher-login";

#[allow(missing_docs)]
/// Angel One API Endpoints
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EndPoint {
    Login,
    Logout,
    Token,
    Refresh,
    UserProfile,

    OrderPlace,
    OrderModify,
    OrderCancel,
    OrderBook,

    LtpData,
    TradeBook,
    RmsLimit,
    Holding,
    Position,
    ConvertPosition,

    GttCreate,
    GttModify,
    GttCancel,
    GttDetails,
    GttList,

    CandleData,
    MarketData,
    SearchScrip,
    AllHolding,
    IndividualOrderDetails(String),
    MarginApi,
}

impl EndPoint {
    /// Returns the login URL
    pub fn login_url<A>(api_key: A) -> String
    where
        A: Display,
    {
        format!("{PUBLISHER_LOGIN}?api_key={api_key}")
    }

    /// Returns the url for the endpoint
    pub fn url(&self) -> String {
        format!("{ROOT_URL}{self}")
    }

    /// Returns the url for the websocket
    pub fn ws() -> String {
        String::from(WS_URL)
    }
}

impl Display for EndPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Login => write!(f, "/rest/auth/angelbroking/user/v1/loginByPassword"),
            Logout => write!(f, "/rest/secure/angelbroking/user/v1/logout"),
            Token => write!(f, "/rest/auth/angelbroking/jwt/v1/generateTokens"),
            Refresh => write!(f, "/rest/auth/angelbroking/jwt/v1/generateTokens"),
            UserProfile => write!(f, "/rest/secure/angelbroking/user/v1/getProfile"),

            OrderPlace => write!(f, "/rest/secure/angelbroking/order/v1/placeOrder"),
            OrderModify => write!(f, "/rest/secure/angelbroking/order/v1/modifyOrder"),
            OrderCancel => write!(f, "/rest/secure/angelbroking/order/v1/cancelOrder"),
            OrderBook => write!(f, "/rest/secure/angelbroking/order/v1/getOrderBook"),

            LtpData => write!(f, "/rest/secure/angelbroking/order/v1/getLtpData"),
            TradeBook => write!(f, "/rest/secure/angelbroking/order/v1/getTradeBook"),
            RmsLimit => write!(f, "/rest/secure/angelbroking/user/v1/getRMS"),
            Holding => write!(f, "/rest/secure/angelbroking/portfolio/v1/getHolding"),
            Position => write!(f, "/rest/secure/angelbroking/order/v1/getPosition"),
            ConvertPosition => write!(f, "/rest/secure/angelbroking/order/v1/convertPosition"),

            GttCreate => write!(f, "/gtt-service/rest/secure/angelbroking/gtt/v1/createRule"),
            GttModify => write!(f, "/gtt-service/rest/secure/angelbroking/gtt/v1/modifyRule"),
            GttCancel => write!(f, "/gtt-service/rest/secure/angelbroking/gtt/v1/cancelRule"),
            GttDetails => write!(f, "/rest/secure/angelbroking/gtt/v1/ruleDetails"),
            GttList => write!(f, "/rest/secure/angelbroking/gtt/v1/ruleList"),

            CandleData => write!(f, "/rest/secure/angelbroking/historical/v1/getCandleData"),
            MarketData => write!(f, "/rest/secure/angelbroking/market/v1/quote/"),
            SearchScrip => write!(f, "/rest/secure/angelbroking/order/v1/searchScrip"),
            AllHolding => write!(f, "/rest/secure/angelbroking/portfolio/v1/getAllHolding"),

            IndividualOrderDetails(order_id) => {
                write!(f, "/rest/secure/angelbroking/order/v1/details/{order_id}")
            }
            MarginApi => write!(f, "/rest/secure/angelbroking/margin/v1/batch"),
        }
    }
}
