//! GP-Assassinator (Gold Pride - Eliminator)
use super::prelude::*;

pub const ID: u32 = 60203670;

const GOLD_PRIDE_TOKEN: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[GOLD_PRIDE_TOKEN], listed_series: &[SET_GOLD_PRIDE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.GetLP(tp)<Duel.GetLP(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::get_lp(d, a.tp) < duel::get_lp(d, 1 - a.tp))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.tkncost)
    effect::set_cost(d, e2, Some(cost_target(tkncost)));
    // e2:SetTarget(s.tkntg)
    effect::set_target(d, e2, Some(cost_target(tkntg)));
    // e2:SetOperation(s.tknop)
    effect::set_operation(d, e2, Some(operation(tknop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOLD_PRIDE) and c:HasLevel()
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_ONFIELD))
    //     and c:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,GOLD_PRIDE_TOKEN,SET_GOLD_PRIDE,TYPES_TOKEN,0,0,c:GetLevel(),RACE_MACHINE,ATTRIBUTE_DARK)
    card::is_set_card(d, c, &[SET_GOLD_PRIDE as u16])
        && card::has_level(d, c)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD))
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let lv = card::get_level(d, c);
            duel::is_player_can_special_summon_monster(d, tp, GOLD_PRIDE_TOKEN, token_data(Some(lv)), None, None, 0)
        }
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,GOLD_PRIDE_TOKEN,SET_GOLD_PRIDE,TYPES_TOKEN,0,0,level,RACE_MACHINE,ATTRIBUTE_DARK)`.
fn token_data(level: Option<u32>) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_GOLD_PRIDE as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level,
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.tkncost(e,tp,...,chk)`
fn tkncost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| costfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| costfilter(d, x, tp)), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(g:GetFirst():GetLevel())
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first);
    effect::set_label(d, a.e, &[lv as i64]);
    true
}
/// `s.tkntg(e,tp,...,chk)`
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.matlimit(e,c,sumtype,tp)`
fn matlimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else { return false };
    let sumtype = a.int(2);
    // local summon_types={SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK}
    // for _,val in pairs(summon_types) do
    //     if val==sumtype then return not c:IsSetCard(SET_GOLD_PRIDE) end
    // end
    let summon_types = [SUMMON_TYPE_FUSION as i64, SUMMON_TYPE_SYNCHRO as i64, SUMMON_TYPE_XYZ as i64, SUMMON_TYPE_LINK as i64];
    if summon_types.contains(&sumtype) {
        return !card::is_set_card(d, c, &[SET_GOLD_PRIDE as u16]);
    }
    // return false
    false
}
/// `s.tknop(e,tp,...)`
fn tknop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
        effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
        // e1:SetValue(s.matlimit)
        effect::set_value_fn(d, e1, lua_fn(matlimit));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e);
    // if not (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,GOLD_PRIDE_TOKEN,SET_GOLD_PRIDE,TYPES_TOKEN,0,0,lv,RACE_MACHINE,ATTRIBUTE_DARK)) then return end
    if !(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, GOLD_PRIDE_TOKEN, token_data(Some(lv as u32)), None, None, 0))
    {
        return;
    }
    // local token=Duel.CreateToken(tp,GOLD_PRIDE_TOKEN)
    let token = duel::create_token(d, tp, GOLD_PRIDE_TOKEN).nil();
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
    effect::set_code(d, e2, EFFECT_CHANGE_LEVEL_FINAL);
    // e2:SetValue(lv)
    effect::set_value(d, e2, lv);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD)
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
    // token:RegisterEffect(e2)
    card::register_effect(d, token, e2, false);
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
