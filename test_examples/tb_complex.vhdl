library ieee;
use ieee.std_logic_1164.all;

entity tb_complex is
end entity;

architecture sim of tb_complex is
  -- Define custom types
  type int_2d_t is array (natural range <>, natural range <>) of integer;
  type slv_1d_t is array (natural range <>) of std_logic_vector(7 downto 0);
  type slv_2d_t is array (natural range <>, natural range <>) of std_logic_vector(3 downto 0);

  type data_record_t is record
    value : integer;
    status : std_logic;
    data : std_logic_vector(7 downto 0);
  end record;

  type record_array_t is array (natural range <>) of data_record_t;

  -- Signal declarations
  signal v_int_2d : int_2d_t(0 to 1, 0 to 2) := ((0, 0, 0), (0, 0, 0));
  signal v_slv_1d : slv_1d_t(0 to 2) := ((others => '0'), (others => '0'), (others => '0'));
  signal v_slv_2d : slv_2d_t(0 to 1, 0 to 1) := (("0000", "0000"), ("0000", "0000"));
  signal v_record : data_record_t := (value => 0, status => '0', data => x"00");
  signal v_record_arr : record_array_t(0 to 1) := ((0, '0', x"00"), (0, '0', x"00"));
begin
  stim : process
  begin
    -- Change 1
    v_int_2d <= ((1, 2, 3), (4, 5, 6));
    v_slv_1d <= (x"AA", x"BB", x"CC");
    v_slv_2d <= (("1010", "0101"), ("1100", "0011"));
    v_record <= (value => 42, status => '1', data => x"A5");
    v_record_arr <= ((42, '1', x"A5"), (10, '0', x"5A"));
    wait for 10 ns;

    -- Change 2
    v_int_2d <= ((10, 20, 30), (40, 50, 60));
    v_slv_1d <= (x"11", x"22", x"33");
    v_slv_2d <= (("1111", "0000"), ("1001", "0110"));
    v_record <= (value => 100, status => '0', data => x"F0");
    v_record_arr <= ((100, '0', x"F0"), (50, '1', x"0F"));
    wait for 10 ns;

    -- Change 3
    v_int_2d <= ((-1, -2, -3), (-4, -5, -6));
    v_slv_1d <= (x"FF", x"00", x"55");
    v_slv_2d <= (("0011", "1100"), ("0101", "1010"));
    v_record <= (value => -1, status => '1', data => x"99");
    v_record_arr <= ((-1, '1', x"99"), (127, '0', x"66"));
    wait for 10 ns;

    wait;
  end process;
end architecture;
