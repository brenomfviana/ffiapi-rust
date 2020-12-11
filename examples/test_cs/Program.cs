using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace csharp
{
  [StructLayout(LayoutKind.Sequential)]
  public struct RustPerson
  {
    public string name;
    public Int32 age;
    public float height;
  };

  // [StructLayout(LayoutKind.Sequential)]
  // public unsafe struct RustPeople
  // {
  //   public Int32 len;
  //   [MarshalAs(UnmanagedType.LPArray, SizeConst=16, ArraySubType=UnmanagedType.Struct)]
  //   public unsafe RustPerson[] data;
  // };

  class Program
  {
    [DllImport("../../target/debug/libffiapi.so")]
    private static extern void print_rust();

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern Int32 addition(Int32 a, Int32 b);

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern string get_str();

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern string concatenate(string s1, string s2);

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern void free_string(string s);

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern RustPerson get_person();

    [DllImport("../../target/debug/libffiapi.so")]
    private static extern void free_person(RustPerson person);

    // [DllImport("../../target/debug/libffiapi.so")]
    // private static extern RustPeople get_people();
    //
    // [DllImport("../../target/debug/libffiapi.so")]
    // private static extern void free_people(RustPeople person);

    static void Main(string[] args)
    {
      // Print in Rust
      Console.WriteLine("Calling function from Rust:");
      print_rust();

      // Sum in Rust
      Console.WriteLine("Sum two numbers from Rust:");
      Console.WriteLine("  1 + 1 = " + addition(1, 1));
      Console.WriteLine("  2 + 2 = " + addition(2, 2));

      // Get string from Rust
      Console.WriteLine("Getting a string from Rust:");
      string rust_str_1 = get_str();
      Console.WriteLine("  String from Rust: '" + rust_str_1 + "'");
      // Concatenate strings in Rust (this not allowed directly in C#)
      Console.WriteLine("Change a Rust string in Rust side:");
      string csharp_str1 = " o/";
      Console.WriteLine("  String from C++: '" + csharp_str1 + "'");
      string rust_str_2 = concatenate(rust_str_1, csharp_str1);
      Console.WriteLine("  Concatenated strings: '" + rust_str_2 + "'");

      // Create an object using a Rust struct
      Console.WriteLine("Creating a person from Rust struct:");
      RustPerson person1;
      person1.name = "A";
      person1.age = 51;
      person1.height = 1.67f;
      Console.WriteLine("  Person 1: ");
      Console.WriteLine("    Name: " + person1.name);
      Console.WriteLine("    Age: " + person1.age);
      Console.WriteLine("    Height: " + person1.height);

      // Get an object from Rust
      Console.WriteLine("Getting a person from Rust function:");
      RustPerson person2 = get_person();
      Console.WriteLine("  Person 2: ");
      Console.WriteLine("    Name: " + person2.name);
      Console.WriteLine("    Age: " + person2.age);
      Console.WriteLine("    Height: " + person2.height);

      // unsafe
      // {
      //   // Get a list of objects from Rust
      //   Console.WriteLine("Getting a list from Rust function:");
      //   RustPeople people = get_people();
      //   int j = 3;
      //   for (int i = 0; i < people.len; i++) {
      //     Console.WriteLine("  Person " + j++);
      //     Console.WriteLine("    Name: " + people.data[i].name);
      //     Console.WriteLine("    Age: " + people.data[i].age);
      //     Console.WriteLine("    Height: " + people.data[i].height);
      //   }
      //   // For some reason, all the frees must be in the end of the method
      //   // Free Rust pointer (all these operation must be in Rust library)
      //   free_people(people);
      // }

      // For some reason, all the frees must be in the end of the method
      // Free Rust pointer (all these operation must be in Rust library)
      free_string(rust_str_1);
      free_string(rust_str_2);
      free_person(person2);
    }
  }
}
