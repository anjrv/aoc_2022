import scala.io.Source

object Solution_1 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var sum = 0

    for (line <- input.getLines()) {
      val sections = line.split("[-,]").map(_.toInt)

      if (
        (sections(0) <= sections(2) && sections(1) >= sections(3))
        || sections(2) <= sections(0) && sections(3) >= sections(1)
      ) {
        sum += 1;
      }
    }

    println(sum)

    input.close
  }
}
