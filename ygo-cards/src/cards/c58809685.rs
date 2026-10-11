//! Elfnotes: Quatrain of Succession
use super::prelude::*;

pub const ID: u32 = 58809685;

/// `local TOKEN_ELFNOTE_SERAPHIM=id+1`
const TOKEN_ELFNOTE_SERAPHIM: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ELFNOTE_SERAPHIM], listed_series: &[SET_ELFNOTE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // The monster in your center Main Monster Zone cannot be banished by your opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e1, EFFECT_CANNOT_REMOVE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetTarget(s.rmlimit)
    effect::set_target(d, e1, Some(lua_fn(rmlimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can send 1 other Spell/Trap from your hand or field to the GY and declare a Level from 1 to 4; ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.tokencost)
    effect::set_cost(d, e2, Some(cost_target(tokencost)));
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmlimit(e,c,tp,r)`
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    // return c:IsSequence(2) and c:IsControler(e:GetHandlerPlayer()) and c:IsLocation(LOCATION_MZONE)
    //     and not c:IsImmuneToEffect(e) and r&REASON_EFFECT>0
    let r = a.int(3) as u32;
    card::is_sequence(d, c, &[2])
        && card::is_controler(d, c, effect::get_handler_player(d, e))
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_immune_to_effect(d, c, e)
        && r & REASON_EFFECT > 0
}
/// `s.tokencostfilter(c,tp)`
fn tokencostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGraveAsCost() and Duel.GetMZoneCount(tp,c)>0
    card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c) && duel::get_m_zone_count(d, tp, (&CardSet::from_card(c)).into(), None, None, None).0 > 0
}
/// `s.tokencost(e,tp,...,chk)`
fn tokencost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tokencostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| tokencostfilter(d, cc, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tokencostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| tokencostfilter(d, cc, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ELFNOTE_SERAPHIM,SET_ELFNOTE,TYPES_TOKEN|TYPE_TUNER,0,0,lv,RACE_PLANT,ATTRIBUTE_FIRE)`
fn token_data(lv: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_ELFNOTE as u16]),
        type_: Some(TYPES_TOKEN | TYPE_TUNER),
        attack: Some(0),
        defense: Some(0),
        level: Some(lv),
        race: Some(RACE_PLANT as u64),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
/// `s.tokentg(e,tp,...,chk)`
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local exceptions={}
    let mut exceptions: Vec<u32> = Vec::new();
    // for lv=1,4 do
    for lv in 1..=4u32 {
        // if not Duel.IsPlayerCanSpecialSummonMonster(...lv...) then table.insert(exceptions,lv) end
        if !duel::is_player_can_special_summon_monster(d, tp, TOKEN_ELFNOTE_SERAPHIM, token_data(lv), Some(POS_FACEUP as u8), None, 0) {
            exceptions.push(lv);
        }
    }
    // if chk==0 then return #exceptions<4 end
    if a.chk == Some(0) {
        return exceptions.len() < 4;
    }
    // local declared_lv=Duel.AnnounceLevel(tp,1,4,exceptions)
    let declared_lv = duel::announce_number_range(d, tp, Some(1), Some(4), &exceptions).nil().0 as i64;
    // e:SetLabel(declared_lv)
    effect::set_label(d, e, &[declared_lv]);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.tokenop(e,tp,...)`
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, e);
    // if Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ELFNOTE_SERAPHIM,...,lv,...) then
    if duel::is_player_can_special_summon_monster(d, tp, TOKEN_ELFNOTE_SERAPHIM, token_data(lv as u32), Some(POS_FACEUP as u8), None, 0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local token=Duel.CreateToken(tp,TOKEN_ELFNOTE_SERAPHIM)
        let token = duel::create_token(d, tp, TOKEN_ELFNOTE_SERAPHIM).nil();
        // Set the Token's Level to the declared Level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
        // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // While that Token is in the Monster Zone, the player who Summoned it cannot Special Summon from the Extra Deck, except "Elfnote" monsters
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD);
            // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
            effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetAbsoluteRange(tp,1,0)
            effect::set_absolute_range(d, e2, tp, 1, 0);
            // e2:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSetCard(SET_ELFNOTE) end)
            effect::set_target(d, e2, Some(effect_card_fn(|d, _e, cc| {
                card::is_location(d, cc, LOCATION_EXTRA) && !card::is_set_card(d, cc, &[SET_ELFNOTE as u16])
            })));
            // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // token:RegisterEffect(e2)
            card::register_effect(d, token, e2, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
