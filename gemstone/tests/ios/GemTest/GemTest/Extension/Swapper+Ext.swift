// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import Gemstone

extension SwapperProviderType: @retroactive CustomStringConvertible {
    public var description: String {
        let json: [String: Any] = [
            "id": String(describing: id),
            "protocol": self.protocol,
        ]
        let bytes = try! JSONSerialization.data(withJSONObject: json, options: [.prettyPrinted, .sortedKeys])
        return String(data: bytes, encoding: .utf8)!
    }
}

extension SwapperQuote: @retroactive CustomStringConvertible {
    public var description: String {
        let provider: [String: Any] = [
            "name": data.provider.name,
            "protocol": data.provider.protocol,
        ]
        let routes: [[String: Any]] = data.routes.map { route in
            [
                "input": route.input,
                "output": route.output,
                "routeData": route.routeData,
                "gasLimit": route.gasLimit ?? "",
            ]
        }
        let json: [String: Any] = [
            "fromValue": fromValue,
            "toValue": toValue,
            "data": [
                "provider": provider,
                "slippageBps": data.slippageBps,
                "routes": routes,
            ],
            "etaInSeconds": etaInSeconds ?? 0,
        ]
        let bytes = try! JSONSerialization.data(withJSONObject: json, options: [.prettyPrinted, .sortedKeys])
        return String(data: bytes, encoding: .utf8)!
    }
}

extension SwapperQuoteData: @retroactive CustomStringConvertible {
    public var description: String {
        var json: [String: Any] = [
            "to": to,
            "value": value,
            "data": data,
            "approval": NSNull(),
            "gasLimit": gasLimit ?? NSNull(),
        ]
        if let approvalData = approval {
            json["approval"] = [
                "token": approvalData.token,
                "spender": approvalData.spender,
                "value": approvalData.value,
            ]
        }
        let bytes = try! JSONSerialization.data(withJSONObject: json, options: [.prettyPrinted, .sortedKeys])
        return String(data: bytes, encoding: .utf8)!
    }
}

public extension SwapperQuoteAsset {
    init(id: String, decimals: UInt32) {
        self.init(
            id: id,
            symbol: "",
            decimals: decimals
        )
    }
}
