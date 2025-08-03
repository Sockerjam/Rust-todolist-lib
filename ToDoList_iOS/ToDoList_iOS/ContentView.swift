import SwiftUI

struct ContentView: View {
    @State private var viewModel = ToDoListViewModel()
    
    var body: some View {
        NavigationStack {
            List {
                ForEach($viewModel.toDoItems, id: \.id) { $item in
                    HStack {
                        Button(action: {
                            viewModel.toggleStatus(item)
                        }) {
                            Image(systemName: viewModel.checkStatus(item) ? "checkmark.circle.fill" : "circle")
                        }
                        TextField("Enter description", text: $item.description, onCommit: {
                            viewModel.updateItem(item)
                        })
                        .strikethrough(viewModel.checkStatus(item))
                        .foregroundColor(viewModel.checkStatus(item) ? .gray : .primary)
                        
                    }
                }
                .onDelete { indexSet in
                    indexSet.forEach {
                        viewModel.removeItem(index: $0)
                    }
                }
            }
            .navigationTitle("To Do List")
            .toolbar {
                Button(action: {
                    viewModel.addItem()
                }) {
                    Image(systemName: "plus")
                }
            }
        }
    }
    
    
}

#Preview {
    ContentView()
}
