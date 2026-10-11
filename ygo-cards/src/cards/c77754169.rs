//! Super Armored Robot Armed Black Iron "C"
//! Updated by DyXel
use super::prelude::*;

pub const ID: u32 = 77754169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand and equip any number of Insect monsters from your GY to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
    // Destroy all monsters your opponent controls with ATK greater than or equal to the ATK of the monster sent to the GY as the cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c,e)`
fn eqfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsCanBeEffectTarget(e) and not c:IsForbidden()
    card::is_race(d, c, RACE_INSECT) && card::is_can_be_effect_target(d, c, Some(e)) && !card::is_forbidden(d, c)
}
/// `s.rescon(g)`: `function(sg,e,tp,mg) return sg:GetClassCount(Card.GetCode)==1 and g:FilterCount(Card.IsCode,nil,sg:GetFirst():GetCode())==3 end`
fn rescon_for(d: &mut Duel, g: &CardSet, sg: &CardSet) -> bool {
    // sg:GetClassCount(Card.GetCode)==1
    if group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) != 1 {
        return false;
    }
    // g:FilterCount(Card.IsCode,nil,sg:GetFirst():GetCode())==3
    let first = group::get_first(d, sg).nil();
    let codes = card::get_code(d, first);
    group::filter_count(d, g, &|d, c| card::is_code(d, c, &codes), NO_EX) == 3
}
/// The `rescon` of `s.rescon(g)` as the callback of `aux.SelectUnselectGroup`
fn rescon_fn(d: &mut Duel, g: &CardSet, sg: &CardSet) -> (bool, bool) {
    (rescon_for(d, g, sg), false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.eqfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && eqfilter(d, chkc, e);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=3 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and ft>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and aux.SelectUnselectGroup(g,e,tp,1,3,s.rescon(g),0) end
    if a.chk == Some(0) {
        return g.len() >= 3
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && ft > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(3), Some(&rescon_fn_for(&g)), 0, tp, None, None, None, false).is_err_and(|x| x);
    }
    // ft=math.min(ft,3)
    ft = ft.min(3);
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(g),1,tp,HINTMSG_EQUIP)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&rescon_fn_for(&g)), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, &tg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(tg), n, tp, 0);
    true
}
/// A `rescon` closure for `g` (`s.rescon(g)`), ready to pass to `aux.SelectUnselectGroup`.
fn rescon_fn_for(g: &CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) + '_ {
    move |d, sg, _e, _tp, _mg, _c| rescon_fn(d, g, sg)
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Match(aux.NOT(Card.IsForbidden),nil)
    let targets = duel::get_target_cards(d, e).nil();
    let mut tg = group::filter(d, &targets, &|d, x| !card::is_forbidden(d, x), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft>0 and #tg>ft then
    if ft > 0 && tg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // tg=tg:Select(tp,ft,ft)
        tg = group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if Duel.Equip(tp,tc,c,true,true) then
        if duel::equip(d, tp, tc, c, true, true) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // --Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
            effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetValue(function(e,c) return e:GetOwner()==c end)
            effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| effect::get_owner(d, e) == Some(c)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
/// `s.descostfilter(c,tp)`
fn descostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:HasFlagEffect(id) and c:IsMonsterCard() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,c:GetTextAttack()),tp,0,LOCATION_MZONE,1,nil)
    let atk = card::get_text_attack(d, c);
    card::has_flag_effect(d, c, ID, None)
        && card::is_monster_card(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(move |d, x| card::is_attack_above(d, x, atk))), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.descost(e,tp,...,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local eqg=e:GetHandler():GetEquipGroup():Filter(s.descostfilter,nil,tp)
    let h = effect::get_handler(d, e).nil();
    let eqg = card::get_equip_group(d, h);
    let eqg = group::filter(d, &eqg, &|d, c| descostfilter(d, c, tp), NO_EX);
    // if chk==0 then return #eqg>0 end
    if a.chk == Some(0) {
        return !eqg.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=eqg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &eqg, tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // e:SetLabel(tc:GetTextAttack())
    let atk = card::get_text_attack(d, tc);
    effect::set_label(d, e, &[atk as i64]);
    // Duel.SendtoGrave(tc,REASON_COST)
    duel::send_to_grave(d, tc, REASON_COST, None, None);
    true
}
/// `s.destg(e,tp,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackAbove,e:GetLabel()),tp,0,LOCATION_MZONE,nil)
    let lbl = effect::get_label(d, a.e) as i32;
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(move |d, x| card::is_attack_above(d, x, lbl))), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackAbove,e:GetLabel()),tp,0,LOCATION_MZONE,nil)
    let lbl = effect::get_label(d, a.e) as i32;
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(move |d, x| card::is_attack_above(d, x, lbl))), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
