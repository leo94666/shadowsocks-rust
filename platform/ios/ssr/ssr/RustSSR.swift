//
//  RustSSR.swift
//  ssr
//
//  Created by li on 2023/12/1.
//

import Foundation

class RustSSR {
    func sslocal(cmd: String,count:Int) {
        // 将 Swift String 转换为 C 字符串
        if let cString = cmd.cString(using: .utf8) {
            // 获取 UnsafeMutablePointer<CChar>
            let unsafePointer = UnsafeMutablePointer(mutating: cString)

            // 在这里使用 unsafePointer
            sslocal_for_ios(unsafePointer,5)
            // 记得释放内存
            unsafePointer.deallocate()
        } else {
            print("Conversion to C string failed.")
        }
    }
}
