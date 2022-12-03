import scala.io.Source

object Solution_2 {
  def toInt(s: String): Int = {
    try {
      s.toInt
    } catch {
      case e: Exception => 0
    }
  }

  // Alternative: Use max heap to add elf sums during getLines,
  // then take sum of 3 dequeues
  // https://www.scala-lang.org/api/2.13.x/scala/collection/mutable/PriorityQueue.html
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var elves = Array(0, 0, 0)
    var curr = 0

    for (line <- input.getLines()) {
      val c = toInt(line);

      if (c > 0)
        curr += c
      else {
        val idx = elves.indexOf(elves.min)
        if (curr > elves(idx))
          elves(idx) = curr;

        curr = 0;
      }
    }

    println(elves.sum);

    input.close
  }
}
