-module(labio).
-export([start/0]).

readlines(FileName) ->
    {ok, Device} = file:open(FileName, [read]),
    try get_all_lines(Device)
    catch _:_:Stacktrace ->
        io:format("Error: ~p~n", [Stacktrace])
    after file:close(Device)
    end.

readFolder(Folder) ->
    {ok, Files} = file:list_dir(Folder),
    io:format("~p -> ~p~n", [Folder, Files]).

get_all_lines(Device) ->
    case io:get_line(Device, "") of
        eof  -> [];
        Line -> readFolder(re:replace(Line, "\n", "", [global, {return, list}])), get_all_lines(Device)
    end.

start() ->
    readlines("folders.txt").