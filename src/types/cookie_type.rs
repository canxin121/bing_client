pub enum Cookie {
    // Json means format like this:
    // [
    // {
    //     "domain": ".bing.com",
    //     "expirationDate": 1743827661.986849,
    //     "hostOnly": false,
    //     "httpOnly": false,
    //     "name": "SnrOvr",
    //     "path": "/",
    //     "sameSite": "no_restriction",
    //     "secure": true,
    //     "session": false,
    //     "storeId": null,
    //     "value": "X=rebateson"
    // },
    // ······
    // ]
    JsonPath(String),
    JsonStr(String),
    // Head means format like:
    // SnrOvr=X=rebateson;SRCHUSR=DOB=20240323&T=1712299341000&TPC=1711617907000&POEX=W; ······
    HeadPath(String),
    HeadStr(String),
}
