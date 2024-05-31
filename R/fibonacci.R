#' Title
#'
#' @param n nth number of fibonacci sequence
#'
#' @return
#' @export
#'
#' @examples
#' fibonacci_r(5)
fibonacci_r<-function(n) {
  f<-c(1,1)
  for (i in 3:n) {
    fn=f[i-1]+f[i-2]
    f[i]<-fn
  }
  return(f)
}

fibonacci_rust<-function(n) {
  n=n+1
  return(n)
}

hello_world_r<-function() {
  print("Hello World from R")
}
