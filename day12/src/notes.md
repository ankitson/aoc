   start
  /  |  \
 a   C   b

  npaths(start,end) = npaths(a,end, visit=any) + npaths(b,end, visit=any + npaths(C,end,visit=any-C)

BFS, but when visiting a capital letter, remove it from the visited set.? increment npaths when end is reached

  start
    /\
c-A--b-d
  \  /
    end

  start-A-end
  start-A-b-end
  start-A-b-A-end
  start-A-b-A-c-A-end
  start-A-c-A-b-A-end
  start-A-c-A-end
  start-A-c-A-b-A-end
  start-A-c-A-b-end
  start-b-end
  start-b-A-c-A-end

Pure BFS cannot work because CAPS-small edges can be traversed twice.



----
  paths = {
    A: [[start,A],[start,b,A]]
    b: [[start,b],[start,A,b]]
    c: [[start,A,c],[start,b,A,c]]
    d: [[start,b,d],[start,A,b,d]]
  }

----

Another approach:
  
  np(p,q) = number of paths(p,q) = sum ( number of paths (p, vi)  number of paths (vi, q) )
    
  we want np(start,end) = sum (np(start,s1),np(start,s2),..,np(start,si)) si = nbr of start
  
  calculate np for all pairs?
  
