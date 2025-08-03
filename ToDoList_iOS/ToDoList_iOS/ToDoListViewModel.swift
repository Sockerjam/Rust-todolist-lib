import SwiftUI
import ToDoList

@Observable
class ToDoListViewModel {
    private let toDoList = ToDoList()
    
    var toDoItems: [ToDoItem] = []
    
    func addItem() {
        toDoList.addItem(description: "", status: .inProgress)
        toDoItems = toDoList.getItems()
    }
    
    func updateItem(_ item: ToDoItem) {
        do {
            try toDoList.updateItem(item: item)
            toDoItems = toDoList.getItems()
        } catch {
            print(error)
        }
    }
    
    func removeItem(index: Int) {
        let item = toDoItems[index]
        
        do {
            try toDoList.deleteItem(item: item)
            toDoItems = toDoList.getItems()
        } catch {
            print(error)
        }
    }
    
    func checkStatus(_ item: ToDoItem) -> Bool {
        item.status == .done
    }
    
    func toggleStatus(_ item: ToDoItem) {
        var item = item
        if item.status == .inProgress {
            item.status = .done
        } else {
            item.status = .inProgress
        }
        updateItem(item)
    }
}
