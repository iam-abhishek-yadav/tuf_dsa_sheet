class Solution {
    findUnion(arr1, arr2, n, m){
        let set = new Set();
        arr1.forEach(num => set.add(num));
        arr2.forEach(num => set.add(num));
        let unionArray = Array.from(set);
        unionArray.sort((a, b) => a - b);
        return unionArray;
    }
}
