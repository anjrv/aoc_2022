import scala.io.Source
import scala.collection.mutable.Stack
import scala.collection.mutable.HashMap

object Solution_2 {
  def toInt(s: String): Int = {
    try {
      s.toInt
    } catch {
      case e: Exception => 0
    }
  }

  def main(args: Array[String]): Unit = {
    val input = Source.fromFile("input.txt")
    val totalSpace = 70000000
    val reqSpace = 30000000

    var directories = new HashMap[String, Int]()
    var current = Stack[String]()
    var size = 0

    for (line <- input.getLines()) {
      val l = line.split(' ')
      size += toInt(l(0))

      if (l(1) == "cd") {
        var dirString = ""
        current.reverseIterator.foreach(dir => {
          dirString = dirString.concat(dir)
          directories(dirString) =
            size + directories.get(dirString).getOrElse(0)
        })

        size = 0

        l(2) match {
          case ".." => current.pop()
          case "/"  => { current.clear(); current.push("/") }
          case _    => current.push(l(2))
        }
      }
    }

    // Clean up last cd
    var dirString = ""
    current.reverseIterator.foreach(dir => {
      dirString = dirString.concat(dir)
      directories(dirString) = size + directories.get(dirString).getOrElse(0)
    })

    println(
      directories
        .filter(dir => dir._2 >= reqSpace - (totalSpace - directories("/")))
        .minBy(_._2)
    )

    input.close
  }
}
