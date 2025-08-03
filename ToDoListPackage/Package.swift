// swift-tools-version: 5.9
import PackageDescription

let package = Package(
  name: "ToDoListDemo",
  platforms: [.iOS(.v15), .macOS(.v12)],
  products: [
    .library(name: "ToDoListDemo", targets: ["ToDoList"])
  ],
  targets: [
    .target(
      name: "ToDoList",
      dependencies: ["core_logicFFI"],
      path: "Sources/CoreLogic"
    ),
    .binaryTarget(
        name: "core_logicFFI",
        path: "Binaries/corelogic.xcframework"
    )
  ]
)
