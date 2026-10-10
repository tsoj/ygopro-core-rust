//! Mare Mare
use super::prelude::*;

pub const ID: u32 = 54537489;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[54537489], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(3,id)
    effect::set_count_limit(d, e2, 3, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let se = a.e(1);
    // return se:IsMonsterEffect() and se:GetHandler():IsRace(RACE_WYRM)
    effect::is_monster_effect(d, se) && {
        let h = effect::get_handler(d, se).nil();
        card::is_race(d, h, RACE_WYRM)
    }
}
/// `s.tokenfilter` data: `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,300,200,1,RACE_WYRM,ATTRIBUTE_WATER)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(300),
            defense: Some(200),
            level: Some(1),
            race: Some(RACE_WYRM),
            attribute: Some(ATTRIBUTE_WATER),
        },
        None,
        None,
        0,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():GetLevel()>1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_level(d, h) > 1
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && can_summon_token(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsFacedown() and c:IsRelateToEffect(e) and (not c:IsImmuneToEffect(e)) and c:UpdateLevel(-1)==-1
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if !card::is_facedown(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && !card::is_immune_to_effect(d, c, a.e)
        && card::update_level(d, c, -1, None, None) == -1
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && can_summon_token(d, tp)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(token), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
