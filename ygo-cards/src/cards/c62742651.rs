//! Armored Axon Kicker
use super::prelude::*;

pub const ID: u32 = 62742651;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon without tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take damage and Special Summon 1 monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DAMAGE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC)`
fn psychic_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    if a.int(2) != 0 {
        return false;
    }
    if card::get_level(d, c) <= 4 {
        return false;
    }
    let p = card::get_controler(d, c);
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_PSYCHIC),c:GetControler(),LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&psychic_faceup), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRelateToBattle() and c:GetBattleTarget():IsMonster()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::is_monster(d, bt)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // local dam=bc:GetAttack()/2
    let mut dam = card::get_attack(d, bc) as f64 / 2.0;
    // if dam<0 then dam=0 end
    if dam < 0.0 {
        dam = 0.0;
    }
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, lua_int(dam) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, lua_int(dam) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp,atk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, atk: i64) -> bool {
    card::is_attack_below(d, c, atk as i32) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local dam=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let dam = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if Duel.Damage(tp,dam,REASON_EFFECT)~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::damage(d, a.tp, dam, REASON_EFFECT, false, None) != 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp,dam)
        let (e, tp) = (a.e, a.tp);
        let nv = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, dam));
        let g = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if #g~=0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
