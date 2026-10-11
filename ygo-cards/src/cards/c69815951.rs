//! Drytron Meteonis Draconids
use super::prelude::*;

pub const ID: u32 = 69815951;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[22398665], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Check material
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.tgval)
    effect::set_value_fn(d, e1, lua_fn(tgval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attack all
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(s.atkfilter)
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, c| card::is_special_summoned(d, c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send targets to grave
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.gycon)
    effect::set_condition(d, e3, Some(condition(gycon)));
    // e3:SetTarget(s.gytg)
    effect::set_target(d, e3, Some(cost_target(gytg)));
    // e3:SetOperation(s.gyop)
    effect::set_operation(d, e3, Some(operation(gyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tgval(e,re,rp)
fn tgval(d: &mut Duel, a: Args) -> bool {
    // return aux.tgoval(e,re,rp) and re:IsMonsterEffect()
    let (e, re, rp) = (a.v(0).clone(), a.v(1).clone(), a.v(2).clone());
    aux::tgoval().call(d, &[e, re.clone(), rp]).truthy() && effect::is_monster_effect(d, re.as_effect().nil())
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:GetSum(Card.GetOriginalLevel)<=2 then
    if group::get_sum(d, &g, &|d, x| card::get_original_level(d, x) as i64) <= 2 {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 1),
        );
    }
    // (no return value: the core reads nil as 0)
    0
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRitualSummoned() and c:GetFlagEffect(id)~=0
    card::is_ritual_summoned(d, c) && card::get_flag_effect(d, c, ID) != 0
}
// function s.gycon(e,tp,eg,ep,ev,re,r,rp)
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and c:GetAttack()>0 and aux.SpElimFilter(c,true)
    card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::get_attack(d, c) > 0
        && aux::sp_elim_filter(d, c, true, false)
}
// function s.rescon(ct)
// return function (sg,e,tp,mg)
fn rescon_with(d: &mut Duel, sg: &CardSet, ct: usize) -> (bool, bool) {
    // local atk=sg:GetSum(Card.GetAttack)
    let atk = group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64);
    if ct == 1 {
        // return atk==2000,atk>2000
        (atk == 2000, atk > 2000)
    } else {
        // return (atk==2000 or atk==4000),atk>4000
        (atk == 2000 || atk == 4000, atk > 4000)
    }
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToGrave()
    card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return s.tgfilter() end
    // (the Lua call passes no card: indexing nil is an error)
    if a.chkc().is_some() {
        crate::lua_error("attempt to index a nil value");
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local ct=Duel.GetMatchingGroupCount(aux.AND(s.tgfilter,Card.IsCanBeEffectTarget),tp,0,LOCATION_ONFIELD,nil,e)
    let ct = duel::get_matching_group_count(
        d,
        Some(&|d, c| tgfilter(d, c) && card::is_can_be_effect_target(d, c, Some(e))),
        tp,
        0,
        LOCATION_ONFIELD,
        NO_EX,
    );
    // if chk==0 then return ct>0 and aux.SelectUnselectGroup(g,e,tp,1,#g,s.rescon(ct),0) end
    if a.chk == Some(0) {
        return ct > 0
            && matches!(
                aux::select_unselect_group(
                    d,
                    &g,
                    Some(e),
                    tp,
                    Some(1),
                    Some(g.len()),
                    Some(&|d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon_with(d, sg, ct)),
                    0,
                    tp,
                    None,
                    None,
                    None,
                    false,
                ),
                Err(true)
            );
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,#g,s.rescon(ct),1,tp,HINTMSG_REMOVE,s.rescon(ct),nil,false)
    let rg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(1),
        Some(g.len()),
        Some(&|d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon_with(d, sg, ct)),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&|d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon_with(d, sg, ct)),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    // local gyc=Duel.GetOperatedGroup():GetSum(Card.GetAttack)//2000
    let operated = duel::get_operated_group(d);
    let gyc = group::get_sum(d, &operated, &|d, c| card::get_attack(d, c) as i64).div_euclid(2000);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sg=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_ONFIELD,gyc,gyc,nil)
    let sg = duel::select_target(d, tp, Some(&tgfilter), tp, 0, LOCATION_ONFIELD, gyc as u16, gyc as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,sg,#sg,0,0)
    let count = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(sg), count, 0, 0);
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
