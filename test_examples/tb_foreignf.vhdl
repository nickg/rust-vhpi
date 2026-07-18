library ieee;
use ieee.std_logic_1164.all;

entity tb_foreignf is
end entity;

architecture sim of tb_foreignf is
    procedure mark_call;
    function add_ints(a : integer; b : integer) return integer;
    procedure bit_reverse_foreign(v_in : in std_logic_vector; v_out : out std_logic_vector);
    function bit_reverse(v : std_logic_vector) return std_logic_vector;

    procedure mark_call is
    begin
        assert false
            report "foreign binding for mark_call was not resolved"
            severity failure;
    end procedure;

    function add_ints(a : integer; b : integer) return integer is
    begin
        assert false
            report "foreign binding for add_ints was not resolved"
            severity failure;
        return 0;
    end function;

    procedure bit_reverse_foreign(v_in : in std_logic_vector; v_out : out std_logic_vector) is
    begin
        assert false
            report "foreign binding for bit_reverse was not resolved"
            severity failure;
    end procedure;

    function bit_reverse(v : std_logic_vector) return std_logic_vector is
        variable ret : std_logic_vector(v'range);
    begin
        bit_reverse_foreign(v, ret);
        return ret;
    end function;

    attribute foreign : string;
    attribute foreign of mark_call : procedure is "VHPI rust_vhpi_tests mark_call";
    attribute foreign of add_ints : function is "VHPI rust_vhpi_tests add_ints";
    attribute foreign of bit_reverse_foreign : procedure is "VHPI rust_vhpi_tests bit_reverse";
begin
    stim : process
        variable result : integer;
        variable rev1 : std_logic_vector(0 downto 0);
        variable rev2 : std_logic_vector(1 downto 0);
        variable rev3 : std_logic_vector(2 downto 0);
        variable rev4 : std_logic_vector(3 downto 0);
        variable rev8 : std_logic_vector(7 downto 0);
        variable rev13 : std_logic_vector(12 downto 0);
    begin
        rev1 := bit_reverse("1");
        assert rev1 = "1"
            report "bit_reverse length-1 failed"
            severity failure;

        rev2 := bit_reverse("10");
        assert rev2 = "01"
            report "bit_reverse length-2 failed"
            severity failure;

        rev3 := bit_reverse("101");
        assert rev3 = "101"
            report "bit_reverse length-3 failed"
            severity failure;

        rev4 := bit_reverse("10X1");
        assert rev4 = "1X01"
            report "bit_reverse length-4 with X failed"
            severity failure;

        rev8 := bit_reverse("10110010");
        assert rev8 = "01001101"
            report "bit_reverse length-8 failed"
            severity failure;

        rev13 := bit_reverse("10ZX01-1U0HWL");
        assert rev13 = "LWH0U1-10XZ01"
            report "bit_reverse length-13 with mixed std_logic values failed"
            severity failure;

        result := add_ints(1, 2);
        assert result = 3 report "add_ints(1, 2) returned " & integer'image(result) severity failure;
        mark_call;
        wait for 5 ns;

        result := add_ints(-5, 7);
        assert result = 2 report "add_ints(-5, 7) returned " & integer'image(result) severity failure;
        mark_call;
        wait for 5 ns;

        result := add_ints(99, -100);
        assert result = -1 report "add_ints(99, -100) returned " & integer'image(result) severity failure;
        mark_call;
        wait for 5 ns;

        result := add_ints(1234, 4321);
        assert result = 5555 report "add_ints(1234, 4321) returned " & integer'image(result) severity failure;
        mark_call;
        wait;
    end process;
end architecture;
