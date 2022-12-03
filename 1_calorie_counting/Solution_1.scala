import scala.io.Source

object Solution_1 {
  def toInt(s: String): Int = {
    try {
      s.toInt
    } catch {
      case e: Exception => 0 
    }
  }

  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var a = 0
    var b = 0

    for (line <- input.getLines()) {
      val c = toInt(line);

      if (c > 0)
        b += c
      else {
        a = Math.max(a, b)
        b = 0;
      }
    }

    println(a)
    input.close
  }
}
