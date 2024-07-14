class Solution{
    public static ArrayList<Integer> findUnion(int arr1[], int arr2[], int n, int m){
        Set<Integer> set = new TreeSet<>();
        for(int i = 0; i < arr1.length; i++){
            set.add(arr1[i]);
        }
        
        for(int i = 0; i < arr2.length; i++){
            set.add(arr2[i]);   
        }
        
        ArrayList<Integer> l=new ArrayList<>(set);
        return l;
    }
}
