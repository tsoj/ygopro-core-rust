//! Black Jack the Shadow-Armored Knight
use super::prelude::*;

pub const ID: u32 = 95506252;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // --Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Excavate cards from the top of your Deck until you excavate a monster, equip it to this card as an Equip Spell, also place the rest on the bottom of the Deck in any order
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(function(c) return c:IsControler(1-tp) and c:IsLocation(LOCATION_STZONE) end,1,nil) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsControler(tp) and ec:IsMonster() end,Card.EquipByEffectAndLimitRegister,e2)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_controler(d, a.c(0), a.p(2)) && card::is_monster(d, a.c(0))),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e2,
        None,
        None,
        None,
    );
    // --Gains ATK/DEF equal to the total Levels of the monsters equipped to this card x 300
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE);
    // e3a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3a, EFFECT_UPDATE_ATTACK);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetValue(function(e,c) return c:GetEquipGroup():Match(Card.HasLevel,nil):GetSum(Card.GetLevel)*300 end)
    effect::set_value_fn(d, e3a, effect_card_value(equip_level_value));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // --If the total Levels of the monsters equipped to this card is greater than 21, destroy this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(function(e) return e:GetHandler():GetEquipGroup():Match(Card.HasLevel,nil):GetSum(Card.GetLevel)>21 end)
    effect::set_condition(d, e4, Some(condition(destroy_cond)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// The total Level of the monsters equipped to `c` (`Match(Card.HasLevel)` then `GetSum(Card.GetLevel)`).
fn equip_level_sum(d: &mut Duel, c: CardId) -> i64 {
    let eg = card::get_equip_group(d, c);
    let lvs = group::filter(d, &eg, &|d, x| card::has_level(d, x), NO_EX);
    group::get_sum(d, &lvs, &|d, x| card::get_level(d, x) as i64)
}
// function(e,c) return c:GetEquipGroup():Match(Card.HasLevel,nil):GetSum(Card.GetLevel)*300 end
fn equip_level_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    equip_level_sum(d, c) * 300
}
// function(e) return e:GetHandler():GetEquipGroup():Match(Card.HasLevel,nil):GetSum(Card.GetLevel)>21 end
fn destroy_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    equip_level_sum(d, h) > 21
}
// function(e,tp,eg) return eg:IsExists(function(c) return c:IsControler(1-tp) and c:IsLocation(LOCATION_STZONE) end,1,nil) end
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, 1 - tp) && card::is_location(d, c, LOCATION_STZONE), 1, NO_EX)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if deck_count==0 then return end
    if deck_count == 0 {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then
    if g.is_empty() {
        // Duel.ConfirmDecktop(tp,deck_count)
        duel::confirm_decktop(d, tp, deck_count as u32);
        // Duel.SortDeckbottom(tp,tp,deck_count)
        duel::sort_deckbottom(d, tp, tp, deck_count as u16);
        // return
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ec=g:GetMaxGroup(Card.GetSequence):GetFirst()
    let (max_g, _) = group::get_max_group(d, &g, &|d, x| card::get_sequence(d, x) as i64).nil();
    let ec = group::get_first(d, &max_g).nil();
    // local ec_seq=ec:GetSequence()
    let ec_seq = card::get_sequence(d, ec) as i32;
    // Duel.ConfirmDecktop(tp,deck_count-ec_seq)
    duel::confirm_decktop(d, tp, (deck_count - ec_seq) as u32);
    // local top_g=Duel.GetDecktopGroup(tp,deck_count-ec_seq)
    let mut top_g = duel::get_decktop_group(d, tp, (deck_count - ec_seq) as u32);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and Duel.GetLocationCount(tp,LOCATION_SZONE,0)>0
    //     and not ec:IsForbidden() and ec:CheckUniqueOnField(tp) then
    if card::is_relate_to_effect(d, c, e)
        && card::is_faceup(d, c)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(0), None, None).0 > 0
        && !card::is_forbidden(d, ec)
        && card::check_unique_on_field(d, ec, tp, None, None)
    {
        // c:EquipByEffectAndLimitRegister(e,tp,ec,nil,true)
        card::equip_by_effect_and_limit_register(d, c, e, tp, ec, None, Some(true));
    } else {
        // Duel.SendtoGrave(ec,REASON_RULE)
        duel::send_to_grave(d, ec, REASON_RULE, None, None);
    }
    // top_g:RemoveCard(ec)
    group::remove_card(&mut top_g, &CardSet::from_card(ec));
    // Duel.MoveToDeckBottom(top_g,tp)
    duel::move_to_deck_bottom(d, top_g.clone());
    // Duel.SortDeckbottom(tp,tp,#top_g)
    duel::sort_deckbottom(d, tp, tp, top_g.len() as u16);
}
