//! Primite Lordly Lode
use super::prelude::*;
use crate::luatable::LuaTable;

pub const ID: u32 = 56506740;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_PRIMITE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Normal Monster with the declared name
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Normal Monsters and "Primite" monsters you control gain 300 ATK for each Normal Monster in your GY with different names
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(function(e,c) return c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_PRIMITE) end)
    effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
    // e3:SetValue(function(e,c) return 300*Duel.GetMatchingGroup(Card.IsType,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil,TYPE_NORMAL):GetClassCount(Card.GetCode) end)
    effect::set_value_fn(d, e3, effect_card_value(e3_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PRIMITE) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PRIMITE as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: Option<u32>) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and (not code or c:IsCode(code))
    if !card::is_type(d, c, TYPE_NORMAL) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None) {
        return false;
    }
    match code {
        None => true,
        Some(code) => card::is_code(d, c, &[code]),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, a.e, a.tp, None)),
                a.tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // local ids={}
    let mut ids: LuaTable<bool> = LuaTable::new();
    // for tc in g:Iter() do ids[tc:GetCode()]=true end
    for tc in group::iter(d, &g) {
        let code = card::get_code(d, tc)[0];
        ids.set(code as i64, Some(true));
    }
    // s.announce_filter={}
    // (only used by the next Duel.AnnounceCard, so it is a local here)
    let mut announce_filter: Vec<u64> = Vec::new();
    // for code,iter in pairs(ids) do
    for (code, _iter) in ids.pairs() {
        // if #s.announce_filter==0 then
        if announce_filter.is_empty() {
            // table.insert(s.announce_filter,code)
            announce_filter.push(code as u64);
            // table.insert(s.announce_filter,OPCODE_ISCODE)
            announce_filter.push(OPCODE_ISCODE);
        } else {
            // table.insert(s.announce_filter,code)
            announce_filter.push(code as u64);
            // table.insert(s.announce_filter,OPCODE_ISCODE)
            announce_filter.push(OPCODE_ISCODE);
            // table.insert(s.announce_filter,OPCODE_OR)
            announce_filter.push(OPCODE_OR);
        }
    }
    // local code=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let code = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(code)
    duel::set_target_param(d, code as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsSpecialSummoned() and re:GetHandler():IsLocation(LOCATION_MZONE) end)
    effect::set_value_fn(d, e1, lua_fn(e1_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,code)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, Some(code)))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `function(e,c) return c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_PRIMITE) end`
fn e3_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_PRIMITE as u16])
}
/// `function(e,c) return 300*Duel.GetMatchingGroup(Card.IsType,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil,TYPE_NORMAL):GetClassCount(Card.GetCode) end`
fn e3_value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_NORMAL)), p, LOCATION_GRAVE, 0, NO_EX);
    300 * group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64
}
/// `function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsSpecialSummoned() and re:GetHandler():IsLocation(LOCATION_MZONE) end`
fn e1_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    if !card::is_special_summoned(d, h) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_location(d, h, LOCATION_MZONE)
}
