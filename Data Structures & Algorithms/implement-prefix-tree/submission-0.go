type PrefixTree struct {
    arr [26]*PrefixTree
    end bool
}

func Constructor() PrefixTree {
    return PrefixTree{
        arr: [26]*PrefixTree{},
        end: false,
    }
}

func (this *PrefixTree) Insert(word string) {
    curr := this
     
    for i := range word {
        r := word[i]
        index := (r - 'a')
        if curr.arr[index] == nil {
            n := Constructor()
            curr.arr[index] = &n
        }

        curr = curr.arr[index]
    }
    curr.end = true
}

func (this *PrefixTree) Search(word string) bool {
    if len(word) == 0 {
        return this.end
    }

    r := word[0]    
    if this.arr[(r - 'a')] == nil {
        return false
    }    
    
    return this.arr[(r - 'a')].Search(word[1:])
}

func (this *PrefixTree) StartsWith(word string) bool {
    if len(word) == 0 {
        return true
    }

    r := word[0]    
    if this.arr[(r - 'a')] == nil {
        return false
    }    
    
    return this.arr[(r - 'a')].StartsWith(word[1:])}
