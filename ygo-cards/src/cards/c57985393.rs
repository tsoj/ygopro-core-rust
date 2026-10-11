//! Broomy
use super::prelude::*;

pub const ID: u32 = 57985393;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be used as Synchro Material, except for the Synchro Summon of a Level 8 or lower monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e0:SetValue(function(e,c) return c and not c:IsLevelBelow(8) end)
    effect::set_value_fn(d, e0, effect_card_fn(|d, _e, c| !card::is_level_below(d, c, 8)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon and banish this card and 1 other monster in your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcostfilter(c,e,tp,rm_chk,sp_chk)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rm_chk: bool, sp_chk: bool) -> bool {
    // return c:IsMonster() and not c:IsPublic()
    //     and ((sp_chk and c:IsAbleToRemove()) or (rm_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_monster(d, c)
        && !card::is_public(d, c)
        && ((sp_chk && card::is_able_to_remove(d, c, None, None, None)) || (rm_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rm_chk=c:IsAbleToRemove()
    let rm_chk = card::is_able_to_remove(d, c, None, None, None);
    // local sp_chk=c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let sp_chk = card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    // if chk==0 then return (rm_chk or sp_chk) and not c:IsPublic()
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c,e,tp,rm_chk,sp_chk) end
    if a.chk == Some(0) {
        return (rm_chk || sp_chk)
            && !card::is_public(d, c)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, e, tp, rm_chk, sp_chk)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp,rm_chk,sp_chk)
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, e, tp, rm_chk, sp_chk)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
/// `s.spfilter(c,e,tp,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and g:IsExists(Card.IsAbleToRemove,1,c)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, g, &|d, x| card::is_able_to_remove(d, x, None, None, None), 1, c.into())
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g==2 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && g.len() == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:FilterSelect(tp,s.spfilter,1,1,nil,e,tp,g)
        let sg = group::filter_select(d, &g, tp, &|d, x| spfilter(d, x, e, tp, &g), 1, 1, false, NO_EX).nil();
        // if #sg==1 then
        if sg.len() == 1 {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // Duel.Remove(g:Sub(sg),POS_FACEUP,REASON_EFFECT)
            let mut rest = g.clone();
            group::remove_card(&mut rest, &sg);
            duel::remove(d, rest, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // --Cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsType(TYPE_SYNCHRO) and c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_SYNCHRO) && card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // --Cannot declare an attack, except with Synchro Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return not c:IsType(TYPE_SYNCHRO) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_SYNCHRO))));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
