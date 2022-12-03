import scala.io.Source

object Solution_2 {
  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    var score = 0;

    for (line <- input.getLines()) {
      val opp = line.charAt(0) - 64
      val goal = line.charAt(2)

      goal match {
        case 'Z' => {
          score += 6

          if (opp == 3)
            score += 1
          else
            score += opp + 1
        }
        case 'Y' => {
          score += opp + 3;
        }
        case 'X' => {
          if (opp == 1)
            score += 3
          else
            score += opp - 1
        }
      }
    }

    println(score)
    input.close
  }
}
