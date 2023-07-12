//
//  ContentView.swift
//  ios-rusty
//
//  Created by TT on 7/11/23.
//

import SwiftUI

func sayHello(to: String) -> String {
    let result = rust_greeting(to)
    let swift_result = String(cString: result!)
    rust_greeting_free(UnsafeMutablePointer(mutating: result))
    return swift_result
}

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text(sayHello(to: "Me!"))
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
