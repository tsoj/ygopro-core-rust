//! Exodia, the Legendary Defender
use super::prelude::*;

pub const ID: u32 = 5008836;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // win
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.winop)
    effect::set_operation(d, e2, Some(operation(winop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon with 5 tribute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SUMMON_PROC);
    // e3:SetCondition(s.ttcon)
    effect::set_condition(d, e3, Some(lua_fn(ttcon)));
    // e3:SetOperation(s.ttop)
    effect::set_operation(d, e3, Some(lua_fn(ttop)));
    // e3:SetValue(SUMMON_TYPE_TRIBUTE+1)
    effect::set_value(d, e3, (SUMMON_TYPE_TRIBUTE + 1) as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // tribute check
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // (e4 is not registered in the Lua script: it is only the label object of e5)
    // give atk/def effect only when summonned
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e5, EFFECT_SUMMON_COST);
    // e5:SetOperation(s.facechk)
    effect::set_operation(d, e5, Some(operation(facechk)));
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.winop(e,tp,eg,ep,ev,re,r,rp)`
fn winop(d: &mut Duel, a: &EffArgs) {
    // local des=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let des = group::get_first(d, &eg).nil();
    // local rc=des:GetReasonCard()
    let rc = card::get_reason_card(d, des);
    // if (des:GetPreviousRaceOnField()&RACE_FIEND==RACE_FIEND and des:GetPreviousAttributeOnField()&ATTRIBUTE_DARK==ATTRIBUTE_DARK
    //     and des:GetOwner()==(1-tp) and rc:IsRelateToBattle() and rc==e:GetHandler() and rc:GetSummonType()==SUMMON_TYPE_TRIBUTE+1)
    let won = (card::get_previous_race_on_field(d, des) & RACE_FIEND) == RACE_FIEND
        && (card::get_previous_attribute_on_field(d, des) & ATTRIBUTE_DARK) == ATTRIBUTE_DARK
        && card::get_owner(d, des) == 1 - a.tp
        && {
            let rc = rc.nil();
            card::is_relate_to_battle(d, rc)
                && Some(rc) == effect::get_handler(d, a.e)
                && card::get_summon_type(d, rc) == SUMMON_TYPE_TRIBUTE + 1
        };
    // then Duel.Win(tp,WIN_REASON_EXODIA_DEFENDER) end
    if won {
        duel::win(d, a.tp, WIN_REASON_EXODIA_DEFENDER);
    }
}
/// `s.ttcon(e,c,minc)`
fn ttcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc<=5 and Duel.CheckTribute(c,5)
    let minc = a.int(2);
    minc <= 5 && duel::check_tribute(d, c, 5, None, None, None, None)
}
/// `s.ttop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ttop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let c = a.c(8);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectTribute(tp,c,5,5)
    let g = duel::select_tribute(d, tp, c, 5, 5, None, Some(tp), None, false).nil();
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g, REASON_SUMMON+REASON_MATERIAL)
    duel::release(d, g, REASON_SUMMON | REASON_MATERIAL, None);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local atk=0
    // local def=0
    let mut atk: i64 = 0;
    let mut def: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local catk=tc:GetTextAttack()
        let catk = card::get_text_attack(d, tc);
        // atk=atk+(catk>=0 and catk or 0)
        atk += if catk >= 0 { catk as i64 } else { 0 };
        // local cdef=tc:GetTextDefense()
        let cdef = card::get_text_defense(d, tc);
        // def=def+(cdef>=0 and cdef or 0)
        def += if cdef >= 0 { cdef as i64 } else { 0 };
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE);
        // e2:SetValue(def)
        effect::set_value(d, e2, def);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    0
}
/// `s.facechk(e,tp,eg,ep,ev,re,r,rp)`
fn facechk(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(1)
    let e4 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e4, &[1]);
}
