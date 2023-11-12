local function mergeArrays(left, right)
    local merged = {}
    local i = 1
    local j = 1
  
    while i <= #left and j <= #right do
      if left[i].age >= right[j].age then
        table.insert(merged, left[i])
        i = i + 1
      else
        table.insert(merged, right[j])
        j = j + 1
      end
    end
  
    while i <= #left do
      table.insert(merged, left[i])
      i = i + 1
    end
  
    while j <= #right do
      table.insert(merged, right[j])
      j = j + 1
    end
  
    return merged
  end
  
  local function calculateMaxAgeDifference(array)
    local maxDifference = 0
  
    for i = 1, #array - 1 do
      local ageDifference = math.abs(array[i].age - array[i + 1].age)
      if ageDifference > maxDifference then
        maxDifference = ageDifference
      end
    end
  
    return maxDifference
  end
  
  local function sortByAge(array)
    if #array <= 1 then
      return array
    end
  
    local mid = math.floor(#array / 2)
    local left = {}
    local right = {}
  
    for i, value in ipairs(array) do
      if i <= mid then
        table.insert(left, value)
      else
        table.insert(right, value)
      end
    end
  
    left = sortByAge(left)
    right = sortByAge(right)
  
    return mergeArrays(left, right)
  end
  
  local function getPersonsFromData(json)
    local arr = {}
    for k, v in pairs(json) do
      if k == "group" then
        local recursionArr = getPersonsFromData(v)
        for _, v in pairs(recursionArr) do
          table.insert(arr, v)
        end
      else
        v["id"] = k
        table.insert(arr, v)
      end
    end
  
    return arr
  end
  
  local data = {
    person1 = {
      name = "John",
      age= 27
    },
    group = {
      person2 = {
        name = "Bob",
        age=35
      },
      person3 = {
        name = "Kevin",
        age = 10
      }
    }
  }
  
  local array = getPersonsFromData(data)
  
  local sortedArray = sortByAge(array)
  
  local maxAgeDifference = calculateMaxAgeDifference(sortedArray)
  
  print("Максимальная разница в возрасте: " .. maxAgeDifference)
  