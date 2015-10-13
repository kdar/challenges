# i dislike 'unless'

def scoring(array)
  array.each do |user|
    if not user.is_admin?
      user.update_score
    end
  end
end
